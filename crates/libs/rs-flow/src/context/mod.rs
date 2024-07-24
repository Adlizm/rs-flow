use std::collections::VecDeque;
use std::{collections::HashMap, sync::Arc};

use crate::context::{ctx::Ctx, global::Global};
use crate::connection::{Connections, Point};
use crate::component::{Id, Component, Type};
use crate::package::Package;
use crate::errors::Errors;

pub mod ctx;
pub mod global;

pub(crate) struct Ctxs<GD> 
    where GD: Send + Sync + 'static
{
    contexts: HashMap<Id, Ctx<GD>>
}
impl<GD> Ctxs<GD> 
    where GD: Send + Sync + 'static
{

    pub(crate) fn new(
        components: &HashMap<Id, Component<GD>>, 
        global: &Arc<Global<GD>>,
    ) -> Self {
        let contexts = components.iter()
            .map(|(id, component)| (*id, Ctx::from(component, &global)))
            .collect();

        Self {
            contexts
        }
    }

    pub(crate) fn lend(&mut self, id: Id) -> Option<Ctx<GD>> {
        if let Some(mut ctx) = self.contexts.remove(&id) {
            ctx.consumed = false;
            return Some(ctx);
        }
        None
    }

    pub(crate) fn give_back(
        &mut self, 
        mut ctx: Ctx<GD>, 
        connections: &Connections
    ) -> Result<(), Errors> {
        let id = ctx.id;
        if ctx.consumed == false {
            return Err(Errors::AnyPackageConsumed { component: id });
        }

        // insert the packages in map or append with the exists packages
        fn insert_or_append(
            point: Point, 
            mut packages: VecDeque<Package>,
            packages_received : &mut HashMap<Point, VecDeque<Package>>
        ) {
            packages_received.entry(point)
                .and_modify(|queue| queue.append(&mut packages))
                .or_insert(packages);
        }
        
        let mut packages_received : HashMap<Point, VecDeque<Package>> = HashMap::new();
        for (port, send_queue) in ctx.send.iter_mut() {
            let mut packages = VecDeque::new();
            std::mem::swap(&mut packages, send_queue);

            if let Some(to_ports) = connections.from(Point::new(id, *port)) {
                match to_ports.len() {
                    0 => {},
                    1 => {
                        let to = to_ports[0].clone();
                        insert_or_append(to, packages, &mut packages_received);
                    },
                    _ => {
                        for i in 1..to_ports.len() {
                            let to = to_ports[i].clone();
                            insert_or_append(to, packages.clone(), &mut packages_received);
                        }
                        let to = to_ports[0].clone();
                        insert_or_append(to, packages, &mut packages_received);
                    }
                }
            }
        }
    
        // insert ctx 
        self.contexts.insert(id, ctx);

        // Puting packages in recieve queue
        for (point, mut packages) in packages_received.drain() {
            if let Some(ctx) = self.contexts.get_mut(&point.id()) {
                if let Some(queue) = ctx.receive.get_mut(&point.port()) {
                    queue.append(&mut packages);
                }
            }
        }

        Ok(())
    }
    
    pub(crate) fn entry_points(&self) -> Vec<Id> {
        self.contexts
            .iter()
            .filter(|(_, component)| component.receive.len() == 0)
            .map(|(id, _)| *id)
            .collect()
    }

    pub(crate) fn ready_components(&mut self, connections: &Connections) -> Vec<Id> {
        let mut ready = self.contexts.iter()
            .filter_map(|(id, ctx)| {
                if ctx.receive.len() == 0 {
                    None
                } else {
                    if ctx.receive.iter().all(|(_, queue)| queue.len() > 0) {
                        Some(*id)
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<Id>>();

            let eager_not_ready = ready.iter()
                .filter(|id| {
                    match self.contexts.get(id)
                        .expect("Ready vec is generted by context map")
                        .ty 
                    {
                        Type::Eager => {
                            connections.any_ancestral_of(&ready, **id)
                        },
                        Type::Lazy => false,
                    }
                })
                .map(|id| *id)
                .collect::<Vec<Id>>();

            ready.retain(|id| !eager_not_ready.contains(&id));

            ready
    }

}