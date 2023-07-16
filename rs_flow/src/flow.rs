use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Mutex, Arc, MutexGuard};

use crate::errors::Errors;
use crate::package::Package;
use crate::component::{Component, ComponentId};
use crate::connection::{Connection, InPoint, OutPoint};
use crate::port::{InPort, OutPort};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlowState { Building, Ready, Running, Finished }
type FlowComponents<'a,T> =  HashMap<ComponentId, &'a mut dyn Component<T>>;
type FlowConnections = HashMap<OutPoint, HashSet<InPoint>>;

type Queues = HashMap<InPoint, Arc<Mutex<VecDeque<Package>>>>;
pub struct ComponentContext<T> {
    id: ComponentId,
    connections: Arc<FlowConnections>,
    global: Arc<Mutex<T>>,
    queues: Arc<Queues>,
}
impl<T> ComponentContext<T> {
    pub fn global(&self) -> Result<MutexGuard<'_, T>, Errors> {
        match self.global.lock() {
            Ok(mutex) => Ok(mutex),
            Err(_) => Err(Errors::CannotLoadGlobal)    
        }
    }
    pub fn receive(&self, in_port: InPort) -> Result<Package, Errors> {
        let in_point = (self.id, in_port.port());
        if let Some(queue) = self.queues.get(&in_point) {
            return match queue.lock() {
                Ok(mut queue) => {
                    if let Some(package) = queue.pop_front() {
                        Ok(package)
                    } else {
                        Err(Errors::EmptyQueue(self.id, in_port.port()))
                    }
                },
                Err(_) => Err(Errors::CannotRecievePackage),
            }
        } else {
            Err(Errors::InPortNotFound(in_point.0, in_point.1))
        }
    }
    pub fn send(&self, out_port: OutPort, package: Package) -> Result<(), Errors> {
        let out_point = (self.id, out_port.port());

        if let Some(in_points) = self.connections.get(&out_point) {
            if in_points.is_empty() {
                return Err(Errors::OutPortNotConnected(self.id, out_port.port()));
            } else {
                for in_point in in_points {    
                    if let Some(queue) = self.queues.get(&in_point) {
                        match queue.lock() {
                            Ok(mut packages) => {
                                packages.push_back(package.clone());
                            },
                            Err(_) => {
                                return Err(Errors::CannotAccessQueue(in_point.0, in_point.1));
                            }
                        }
                    } else {
                        return Err(Errors::QueueNotCreated(in_point.0, in_point.1));  
                    }
                }
            }
            Ok(())
        } else {
            Err(Errors::OutPortNotFound(self.id, out_port.port()))
        }
    }
}

pub struct Flow<'a, T> {
    components: FlowComponents<'a, T>,
    connections: FlowConnections,
    state: FlowState,
}

type ReadyComponents = HashSet<ComponentId>;
impl<'a, T> Flow<'a, T> {
    pub fn new() -> Self {
        Self { 
            components: FlowComponents::new(),
            connections: FlowConnections::new(),
            state: FlowState::Building,
        }
    }
    pub fn add_component(&mut self, component: &'a mut dyn Component<T>) -> Result<&mut Self, Errors> {
        if  self.state != FlowState::Building {
            return Err(Errors::FlowAlreadyBuilded);
        }
        if self.components.contains_key(&component.id()) {
            return Err(Errors::ComponentAlreadyExist(component.id()))
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
            let found = outputs.iter().any(|port| port.port() == connection.out_port);
            if !found {
                return Err(Errors::OutPortNotFound(connection.from, connection.out_port));
            }
        } else {
            return Err(Errors::ComponentNotFound(connection.from));
        }
        if let Some(component) = self.components.get(&connection.to) {
            let inputs = component.inputs();
            let found = inputs.iter().any(|port| port.port() == connection.in_port);
            if !found {
                return Err(Errors::OutPortNotFound(connection.from, connection.out_port));
            }
        } else {
            return Err(Errors::ComponentNotFound(connection.to));
        }

        if let Some(in_ports) = self.connections.get_mut(&(connection.from, connection.out_port)) {
            in_ports.insert((connection.to, connection.in_port));
        } else {
            let mut in_ports = HashSet::new();
            in_ports.insert((connection.to, connection.in_port));

            self.connections.insert((connection.from, connection.out_port), in_ports);
        }
        Ok(self)
    }
    pub fn build(&mut self) -> Result<(), Errors> {
        if  self.state != FlowState::Building {
            return Err(Errors::FlowAlreadyBuilded);
        }
        self.state = FlowState::Ready;
        return Ok(());
    }

    pub fn state(&self) -> FlowState {
        self.state
    }

    pub fn run(&mut self, global: T) -> Result<(), Errors> {
        if self.state == FlowState::Building { return Err(Errors::FlowUnreadyToRun); }
        if self.state == FlowState::Running { return Err(Errors::FlowAlreadyRunning); }

        self.state = FlowState::Ready;

        let queues = Arc::new(self.create_queues());
        let global = Arc::new(Mutex::new(global));
        let connections = Arc::new(self.connections.clone());
        for component in self.components.values_mut() {
            let context = ComponentContext {
                id: component.id(),
                connections: Arc::clone(&connections),
                global: Arc::clone(&global),
                queues: Arc::clone(&queues)
            };
            component.set_context(context);
        }

        //entry points, all components without inputs
        let mut ready_components = self.entry_points();

        self.state = FlowState::Running;
        while !ready_components.is_empty() {
            for id in ready_components {
                if let Some(component) = self.components.get_mut(&id) {
                    component.run()?;   
                }
            }
            ready_components = self.components_ready_to_run(&queues)?;
        }

        self.state = FlowState::Finished;
        Ok(())
    }

    fn create_queues(&self) -> Queues {
        let mut queues = HashMap::new();
        for (_, in_points) in &self.connections  {
            for in_point in in_points {
                if !queues.contains_key(in_point) {
                    queues.insert(*in_point, Arc::new(Mutex::new(VecDeque::new())));
                }
            }
        }
        return queues;
    }
    fn components_ready_to_run(&self, queues: &Arc<Queues>) -> Result<ReadyComponents, Errors> {
        let mut ready_components= HashSet::new();
        // filter components with some input
        for (_, in_points) in &self.connections {
            for (id, _) in in_points {
                if !ready_components.contains(id) {
                    ready_components.insert(*id);
                }
            }
        }
        // filter components without packages in a queue from input port
        for ((id, port), queue) in queues.iter() {
            if ready_components.contains(id) {
                match queue.lock() {
                    Ok(queue) => {
                        if queue.is_empty() {
                            ready_components.remove(id);
                        }
                    },
                    Err(_) => { return Err(Errors::CannotAccessQueue(*id, *port))}
                }
            }
        }
        Ok(ready_components)
    }
    fn entry_points(&self) ->  ReadyComponents {
        let mut entry_points = HashSet::from_iter(self.components.keys().map(|value| *value));
        for in_points in self.connections.values() {
            for (componenent, _) in in_points {
                entry_points.remove(componenent);
            }
        }
        return entry_points;
    }
}