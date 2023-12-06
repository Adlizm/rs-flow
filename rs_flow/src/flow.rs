use crate::component::ComponentHandler;
use crate::connection::Connection;
use crate::context::queues::Queues;
use crate::context::{ContextPart, Ctx};
use crate::errors::Errors;
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

    pub fn add_component(mut self, component: Box<dyn ComponentHandler>) -> Result<Self, Errors> {
        if self.components.iter().any(|c| c.id() == component.id()) {
            return Err(Errors::ComponentAlreadyExist { id: component.id() });
        }
        self.components.push(component);
        Ok(self)
    }

    pub fn add_connection(mut self, connection: Connection) -> Result<Self, Errors> {
        if self.connections.iter().any(|conn| conn.eq(&connection)) {
            return Err(Errors::ConnectionAlreadyExist(connection));
        }

        let from = self.components.iter().find(|c| c.id() == connection.from);
        if let Some(component) = from {
            if !component
                .outputs()
                .iter()
                .any(|port| port.port == connection.out_port)
            {
                return Err(Errors::OutPortNotFound(connection.out_point()));
            }
        } else {
            return Err(Errors::ComponentNotFound {
                id: connection.from,
            });
        }

        let to = self.components.iter().find(|c| c.id() == connection.to);
        if let Some(component) = to {
            if !component
                .inputs()
                .iter()
                .any(|port| port.port == connection.in_port)
            {
                return Err(Errors::InPortNotFound(connection.in_point()));
            }
        } else {
            return Err(Errors::ComponentNotFound { id: connection.to });
        }

        self.connections.push(connection);
        Ok(self)
    }

    pub async fn run(&mut self) -> Result<(), Errors> {
        let part = ContextPart::from(&self.connections);

        //entry points, all components without inputs
        let mut ready_components = self
            .components
            .iter()
            .filter(|component| component.inputs().is_empty())
            .collect::<Vec<_>>();

        while !ready_components.is_empty() {
            for component in ready_components {
                let ctx = Ctx::from(component.id(), &part);
                component.run(&ctx).await?;
            }

            let has_packages = part.queues.has_packages()?;
            ready_components = self
                .components
                .iter()
                .filter(|component| {
                    if component.inputs().is_empty() {
                        return false;
                    } else {
                        let id = component.id();
                        let ready = component
                            .inputs()
                            .iter()
                            .all(|port| has_packages.contains(&Point::new(id, port.port)));
                        ready
                    }
                })
                .collect();
        }

        Ok(())
    }
}
