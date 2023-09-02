use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::component::Id;
use crate::connection::Point;
use crate::errors::Errors;
use crate::package::Package;
use crate::port::{InPort, OutPort};

pub(crate) type Queues = HashMap<Point, Arc<Mutex<VecDeque<Package>>>>;

pub struct Ctx {
    id: Id,
    connections: Arc<HashMap<Point, HashSet<Point>>>,
    queues: Arc<Queues>,
}

impl Ctx {
    pub fn new(
        id: Id,
        connections: &Arc<HashMap<Point, HashSet<Point>>>,
        queues: &Arc<Queues>,
    ) -> Self {
        Self {
            id,
            connections: connections.clone(),
            queues: queues.clone(),
        }
    }

    pub fn receive(&self, in_port: InPort) -> Result<Package, Errors> {
        let in_point = Point::new(self.id, in_port.port());

        if let Some(queue) = self.queues.get(&in_point) {
            return match queue.lock() {
                Ok(mut queue) => {
                    if let Some(package) = queue.pop_front() {
                        Ok(package)
                    } else {
                        Err(Errors::EmptyQueue(in_point))
                    }
                }
                Err(_) => Err(Errors::CannotRecievePackage),
            };
        } else {
            Err(Errors::InPortNotFound(in_point))
        }
    }

    pub fn send(&self, out_port: OutPort, package: Package) -> Result<(), Errors> {
        let out_point = Point::new(self.id, out_port.port());

        if let Some(in_points) = self.connections.get(&out_point) {
            if in_points.is_empty() {
                return Err(Errors::OutPortNotConnected(out_point));
            } else {
                for in_point in in_points {
                    if let Some(queue) = self.queues.get(&in_point) {
                        match queue.lock() {
                            Ok(mut packages) => {
                                packages.push_back(package.clone());
                            }
                            Err(_) => {
                                return Err(Errors::CannotAccessQueue(in_point.clone()));
                            }
                        }
                    } else {
                        return Err(Errors::QueueNotCreated(in_point.clone()));
                    }
                }
            }
            Ok(())
        } else {
            Err(Errors::OutPortNotFound(out_point))
        }
    }
}
