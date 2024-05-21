use std::sync::Arc;

use crate::connection::{Point, Connection};
use crate::context::Ctx;
use crate::context::part::ContextPart;
use crate::errors::{Errors, Result};
use crate::prelude::Component;

pub struct Flow<GD> 
    where GD: Sync + Send
{
    components: Vec<Component<GD>>,
    connections: Vec<Connection>,
}


impl<GD> Flow<GD> 
    where GD: Sync + Send + 'static
{
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_component(mut self, component: Component<GD>) -> Result<Self> {
        if self.components.iter().any(|c| c.id == component.id) {
            return Err(Errors::ComponentAlreadyExist { id: component.id }.into());
        }
        self.components.push(component);
        Ok(self)
    }

    pub fn add_connection(mut self, connection: Connection) -> Result<Self> {
        if self.connections.iter().any(|conn| conn.eq(&connection)) {
            return Err(Errors::ConnectionAlreadyExist(connection).into());
        }

        let from = self.components.iter().find(|c| c.id == connection.from);
        if let Some(component) = from {
            if !component.data.outputs().contains(connection.out_port)
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

        let to = self.components.iter().find(|c| c.id == connection.to);
        if let Some(component) = to {
            if !component.data.inputs().contains(connection.in_port)
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


    pub async fn run(&self, global: GD) -> Result<GD> {
        let part = ContextPart::from(&self.connections, global);
        let part = Arc::new(part);
        
        //entry points, all components without inputs
        let mut ready_components = self.entry_points();

        while !ready_components.is_empty() {
            for component in ready_components {
                let ctx = Ctx::from(component.id, &part);
                component.data.run(ctx).await?;
            }

            let has_packages = part.queues.has_packages()?;
            ready_components = self.ready_components(has_packages);
        }
        
        let global = Arc::try_unwrap(part)
            .expect("Arc parts have multiples owners, something wrong")
            .global
            .take();
        Ok(global)
    }

    fn entry_points(&self) -> Vec<&Component<GD>> {
        self.components
            .iter()
            .filter(|component| component.data.inputs().is_empty())
            .collect()
    }
    fn ready_components(&self, has_packages: Vec<Point>) -> Vec<&Component<GD>> {
        self.components
            .iter()
            .filter(|component| {
                let inputs = component.data.inputs();
                if inputs.is_empty() {
                    // entry points, only once run
                    return false;
                } else {
                    let id = component.id;
                    let ready = inputs
                        .all(|port| has_packages.contains(&Point::new(id, port.port)));
                    ready
                }
            })
            .collect()
    }
}