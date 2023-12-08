use crate::component::ComponentHandler;
use crate::connection::Connection;
use crate::context::queues::Queues;
use crate::context::{ContextPart, Ctx};
use crate::errors::{Errors, Result};
use crate::prelude::Point;

pub struct Flow {
    components: Vec<Box<dyn ComponentHandler>>,
    connections: Vec<Connection>,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_component(mut self, component: Box<dyn ComponentHandler>) -> Result<Self> {
        if self.components.iter().any(|c| c.id() == component.id()) {
            return Err(Errors::ComponentAlreadyExist { id: component.id() }.into());
        }
        self.components.push(component);
        Ok(self)
    }

    pub fn add_connection(mut self, connection: Connection) -> Result<Self> {
        if self.connections.iter().any(|conn| conn.eq(&connection)) {
            return Err(Errors::ConnectionAlreadyExist(connection).into());
        }

        let from = self.components.iter().find(|c| c.id() == connection.from);
        if let Some(component) = from {
            if !component
                .outputs()
                .iter()
                .any(|port| port.port == connection.out_port)
            {
                return Err(Errors::OutPortNotFound {
                    component: connection.from,
                    out_port: connection.out_port,
                }
                .into());
            }
        } else {
            return Err(Errors::ComponentNotFound {
                id: connection.from,
            }
            .into());
        }

        let to = self.components.iter().find(|c| c.id() == connection.to);
        if let Some(component) = to {
            if !component
                .inputs()
                .iter()
                .any(|port| port.port == connection.in_port)
            {
                return Err(Errors::InPortNotFound {
                    component: connection.from,
                    in_port: connection.in_port,
                }
                .into());
            }
        } else {
            return Err(Errors::ComponentNotFound { id: connection.to }.into());
        }

        self.connections.push(connection);
        Ok(self)
    }

    pub async fn run(&mut self) -> Result<()> {
        let part = ContextPart::from(&self.connections);

        //entry points, all components without inputs
        let mut ready_components = self.entry_points();

        while !ready_components.is_empty() {
            for component in ready_components {
                let ctx = Ctx::from(component.id(), &part);
                component.run(&ctx).await?;
            }

            let has_packages = part.queues.has_packages()?;
            ready_components = self.ready_components(has_packages);
        }

        Ok(())
    }

    fn entry_points(&self) -> Vec<&Box<dyn ComponentHandler>> {
        self.components
            .iter()
            .filter(|component| component.inputs().is_empty())
            .collect()
    }
    fn ready_components(&self, has_packages: Vec<Point>) -> Vec<&Box<dyn ComponentHandler>> {
        self.components
            .iter()
            .filter(|component| {
                let inputs = component.inputs();
                if inputs.is_empty() {
                    // entry points, only once run
                    return false;
                } else {
                    let id = component.id();
                    let ready = inputs
                        .iter()
                        .all(|port| has_packages.contains(&Point::new(id, port.port)));
                    ready
                }
            })
            .collect()
    }
}
