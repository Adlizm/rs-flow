use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::component::{ComponentHandler, Id};
use crate::connection::{Connection, Point};
use crate::context::{Ctx, Queues};
use crate::errors::Errors;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlowState {
    Building,
    Ready,
    Running,
    Finished,
}

type ReadyComponents = HashSet<Id>;

pub struct Flow {
    components: HashMap<Id, Box<dyn ComponentHandler>>,
    connections: HashMap<Point, HashSet<Point>>,
    state: FlowState,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            connections: HashMap::new(),
            state: FlowState::Building,
        }
    }

    pub fn add_component(
        &mut self,
        component: Box<dyn ComponentHandler>,
    ) -> Result<&mut Self, Errors> {
        if self.state != FlowState::Building {
            return Err(Errors::FlowAlreadyBuilded);
        }
        if self.components.contains_key(&component.id()) {
            return Err(Errors::ComponentAlreadyExist { id: component.id() });
        }
        self.components.insert(component.id(), component);
        Ok(self)
    }

    pub fn add_connection(&mut self, connection: Connection) -> Result<&mut Self, Errors> {
        if self.state != FlowState::Building {
            return Err(Errors::FlowAlreadyBuilded);
        }
        if let Some(component) = self.components.get(&connection.from) {
            let outputs = component.outputs();
            let found = outputs
                .iter()
                .any(|port| port.port() == connection.out_port);
            if !found {
                return Err(Errors::OutPortNotFound(connection.out_point()));
            }
        } else {
            return Err(Errors::ComponentNotFound {
                id: connection.from,
            });
        }
        if let Some(component) = self.components.get(&connection.to) {
            let inputs = component.inputs();
            let found = inputs.iter().any(|port| port.port() == connection.in_port);
            if !found {
                return Err(Errors::OutPortNotFound(connection.out_point()));
            }
        } else {
            return Err(Errors::ComponentNotFound { id: connection.to });
        }

        if let Some(in_ports) = self.connections.get_mut(&connection.out_point()) {
            in_ports.insert(connection.in_point());
        } else {
            let mut in_ports = HashSet::new();
            in_ports.insert(connection.in_point());

            self.connections.insert(connection.out_point(), in_ports);
        }
        Ok(self)
    }

    pub fn build(&mut self) -> Result<(), Errors> {
        if self.state != FlowState::Building {
            return Err(Errors::FlowAlreadyBuilded);
        }
        self.state = FlowState::Ready;
        return Ok(());
    }

    pub fn state(&self) -> FlowState {
        self.state
    }

    pub async fn run(&mut self) -> Result<(), Errors> {
        if self.state == FlowState::Building {
            return Err(Errors::FlowUnreadyToRun);
        }
        if self.state == FlowState::Running {
            return Err(Errors::FlowAlreadyRunning);
        }

        self.state = FlowState::Ready;

        let queues = Arc::new(self.create_queues());
        let connections = Arc::new(self.connections.clone());

        //entry points, all components without inputs
        let mut ready_components = self.entry_points();

        self.state = FlowState::Running;
        while !ready_components.is_empty() {
            for id in ready_components {
                let component = self
                    .components
                    .get(&id)
                    .ok_or_else(|| Errors::ComponentNotFound { id })?;

                let ctx = Ctx::new(component.id(), &connections, &queues);
                component.run(&ctx).await?;
            }
            ready_components = self.components_ready_to_run(&queues)?;
        }

        self.state = FlowState::Finished;
        Ok(())
    }

    fn create_queues(&self) -> Queues {
        let mut queues = HashMap::new();
        for (_, in_points) in &self.connections {
            for in_point in in_points {
                if !queues.contains_key(in_point) {
                    queues.insert(in_point.clone(), Arc::new(Mutex::new(VecDeque::new())));
                }
            }
        }
        return queues;
    }
    fn components_ready_to_run(&self, queues: &Arc<Queues>) -> Result<ReadyComponents, Errors> {
        let mut ready_components = HashSet::new();
        // filter components with some input
        for (_, in_points) in &self.connections {
            for in_point in in_points {
                if !ready_components.contains(&in_point.id()) {
                    ready_components.insert(in_point.id());
                }
            }
        }
        // filter components without packages in a queue from input port
        for (in_point, queue) in queues.iter() {
            if ready_components.contains(&in_point.id()) {
                match queue.lock() {
                    Ok(queue) => {
                        if queue.is_empty() {
                            ready_components.remove(&in_point.id());
                        }
                    }
                    Err(_) => return Err(Errors::CannotAccessQueue(in_point.clone())),
                }
            }
        }
        Ok(ready_components)
    }
    fn entry_points(&self) -> ReadyComponents {
        let mut entry_points = HashSet::from_iter(self.components.keys().map(|value| *value));
        for in_points in self.connections.values() {
            for in_point in in_points {
                entry_points.remove(&in_point.id());
            }
        }
        return entry_points;
    }
}
