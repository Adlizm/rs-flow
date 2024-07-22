use std::collections::VecDeque;
use std::{collections::HashMap, sync::Arc};

use crate::context::{ctx::Ctx, global::Global};
use crate::connection::{Connections, Point};
use crate::component::{Id, Component, Next, Type};
use crate::prelude::Package;

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

    pub(crate) fn pop(&mut self, id: Id) -> Option<Ctx<GD>> {
        self.contexts.remove(&id)
    }

    pub(crate) fn refresh(
        &mut self, 
        ctxs: Vec<(Ctx<GD>, Next)>, 
        connections: &Connections
    ) {
        // insert ctxs 
        for (ctx, _) in ctxs {
            self.contexts.insert(ctx.id, ctx);
        }

        let mut packages_received : HashMap<Point, VecDeque<Package>> = HashMap::new();

        // insert the packages in map or append with the exists packages
        fn insert_or_append(
            point: Point, 
            mut packages: VecDeque<Package>,
            packages_received : &mut HashMap<Point, VecDeque<Package>>
        ) {
            if packages_received.contains_key(&point) {
                packages_received.get_mut(&point).unwrap().append(&mut packages);
            } else {
                packages_received.insert(point, packages);
            }
        }

        // Collecting packeges from send queues
        for (&id, ctx) in self.contexts.iter_mut() {
            for (&port, send_queue) in ctx.send.iter_mut() {
                if send_queue.is_empty() {
                    break;
                }
                let mut packages = VecDeque::new();

                std::mem::swap(&mut packages, send_queue);

                if let Some(to_ports) = connections.from(Point::new(id, port)) {
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
        }
    
        // Puting packages in recieve queue
        for (point, mut packages) in packages_received.drain() {
            if let Some(ctx) = self.contexts.get_mut(&point.id()) {
                if let Some(queue) = ctx.receive.get_mut(&point.port()) {
                    queue.append(&mut packages);
                }
            }
        }
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