use std::collections::HashMap;
use std::sync::Arc;

use crate::component::Next;
use crate::connection::{Connection, Connections};
use crate::context::global::Global;
use crate::context::Ctxs;
use crate::errors::{Errors, Result};
use crate::prelude::{Component, Id};

pub struct Flow<GD> 
    where GD: Sync + Send
{
    components: HashMap<Id, Component<GD>>,
    connections: Connections,
}


impl<GD> Flow<GD> 
    where GD: Sync + Send + 'static
{
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            connections: Connections::new(),
        }
    }

    pub fn add_component(mut self, component: Component<GD>) -> Result<Self> {
        if self.components.contains_key(&component.id) {
            return Err(Errors::ComponentAlreadyExist { id: component.id }.into());
        }
        self.components.insert(component.id ,component);
        Ok(self)
    }

    pub fn add_connection(mut self, connection: Connection) -> Result<Self> {
        if let Some(component) = self.components.get(&connection.from) {
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

        if let Some(component) = self.components.get(&connection.to){
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

        self.connections.add(connection)?;

        Ok(self)
    }


    pub async fn run(&self, global: GD) -> Result<GD> {
        let global_arc = Arc::new(Global::from_data(global));
        
        let mut contexts = Ctxs::new(&self.components, &global_arc);

        let mut ready_components = contexts.entry_points();

        while !ready_components.is_empty() {
            let mut futures = Vec::with_capacity(ready_components.len());

            for id in ready_components {
                let mut ctx = contexts.pop(id)
                    .expect("Ready component never return ids that not exist");

                let component = self.components.get(&id)
                    .expect("Ready component never return ids that not exist");

                futures.push(async move {
                    component.data.run(&mut ctx).await
                        .map(|next| (ctx, next))
                });
            }

            let results = futures::future::try_join_all(futures).await?;
            if results.iter().any(|(_, next)| next == &Next::Break) {
                break;
            }

            contexts.refresh(results, &self.connections);

            ready_components = contexts.ready_components(&self.connections);
        }
        
        drop(contexts);
        
        let global = Arc::try_unwrap(global_arc)
            .expect("Global have multiples owners, something wrong")
            .take();
        Ok(global)
    }
}