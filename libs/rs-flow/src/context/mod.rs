use std::collections::VecDeque;
use std::{collections::HashMap, sync::Arc};

use crate::component::{Component, Id, Type};
use crate::connection::{Connections, Point};

mod ctx;
pub use ctx::Ctx;

mod global;
pub use global::Global;

pub(crate) struct Ctxs<V> {
    connections: Connections,
    contexts: HashMap<Id, Ctx<V>>,
}
impl<V> Ctxs<V>
where
    V: Send + Clone,
{
    pub(crate) fn new(
        components: &HashMap<Id, Component<V>>,
        connections: &Connections,
        global: &Arc<Global>,
    ) -> Self {
        Self {
            connections: connections.clone(),
            contexts: components
                .iter()
                .map(|(id, component)| (*id, Ctx::from(component, Arc::clone(global))))
                .collect(),
        }
    }

    pub(crate) fn borrow(&mut self, id: Id) -> Option<Ctx<V>> {
        self.contexts.remove(&id)
    }

    pub(crate) fn refresh_queues(&mut self) {
        // insert the packages in map or append with the exists packages
        fn insert_or_append<V>(
            point: Point,
            mut packages: VecDeque<V>,
            packages_received: &mut HashMap<Point, VecDeque<V>>,
        ) {
            packages_received
                .entry(point)
                .and_modify(|queue| queue.append(&mut packages))
                .or_insert(packages);
        }

        let mut packages_received: HashMap<Point, VecDeque<V>> = HashMap::new();

        for (id, ctx) in self.contexts.iter_mut() {
            for (port, send_queue) in ctx.send.iter_mut() {
                if send_queue.is_empty() {
                    continue;
                }

                let mut packages = VecDeque::new();
                std::mem::swap(&mut packages, send_queue);

                if let Some(to_ports) = self.connections.from(Point::new(*id, *port)) {
                    match to_ports.len() {
                        0 => {}
                        1 => {
                            let to = to_ports[0].clone();
                            insert_or_append::<V>(to, packages, &mut packages_received);
                        }
                        _ => {
                            for i in 1..to_ports.len() {
                                let to = to_ports[i].clone();
                                insert_or_append::<V>(to, packages.clone(), &mut packages_received);
                            }
                            let to = to_ports[0].clone();
                            insert_or_append::<V>(to, packages, &mut packages_received);
                        }
                    }
                }
            }
        }

        // Puting packages in recieve queue
        for (point, mut packages) in packages_received.drain() {
            if let Some(ctx) = self.contexts.get_mut(&point.id()) {
                if let Some(queue) = ctx.receive.get_mut(&point.port()) {
                    queue.push_all(&mut packages);
                }
            }
        }
    }

    pub(crate) fn give_back(&mut self, ctx: Ctx<V>) {
        self.contexts.insert(ctx.id, ctx);
    }

    pub(crate) fn entry_points(&self) -> Vec<Id> {
        self.contexts
            .iter()
            .filter(|(_, component)| component.receive.len() == 0)
            .map(|(id, _)| *id)
            .collect()
    }

    pub(crate) fn ready_components(&mut self, connections: &Connections) -> Vec<Id> {
        let mut ready = self
            .contexts
            .iter()
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

        let eager_not_ready = ready
            .iter()
            .filter(|id| {
                match self
                    .contexts
                    .get(id)
                    .expect("Ready vec is generted by context map")
                    .ty
                {
                    Type::Eager => connections.is_any_of_ancestors(**id, &ready),
                    Type::Lazy => false,
                }
            })
            .map(|id| *id)
            .collect::<Vec<Id>>();

        ready.retain(|id| !eager_not_ready.contains(&id));

        ready
    }
}
