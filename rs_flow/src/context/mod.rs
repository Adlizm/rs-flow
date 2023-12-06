use std::sync::Arc;

use connection::Connection;

use crate::component::Id;
use crate::connection::{self, Point};
use crate::errors::Errors;
use crate::package::Package;
use crate::prelude::Port;

pub mod queues;
pub use queues::*;
pub(crate) struct ContextPart<T: queues::Queues> {
    connections: Vec<Connection>,
    pub(crate) queues: T,
}

impl<T> ContextPart<T>
where
    T: queues::Queues,
{
    pub(crate) fn from(connections: &Vec<Connection>) -> Arc<Self> {
        Arc::new(Self {
            connections: connections.to_vec(),
            queues: T::from(connections),
        })
    }
}

pub struct Ctx<T: queues::Queues> {
    id: Id,
    part: Arc<ContextPart<T>>,
}

impl<T> Ctx<T>
where
    T: queues::Queues,
{
    pub(crate) fn from(id: Id, part: &Arc<ContextPart<T>>) -> Self {
        Self {
            id,
            part: part.clone(),
        }
    }

    pub fn receive(&self, in_port: Port) -> Result<Package, Errors> {
        let in_point = Point::new(self.id, in_port.port);
        self.part.queues.receive(in_point)
    }

    pub fn send(&self, out_port: Port, package: Package) -> Result<(), Errors> {
        let out_point = Point::new(self.id, out_port.port);

        let in_points: Vec<Point> = self
            .part
            .connections
            .iter()
            .filter(|conn| conn.out_point() == out_point)
            .map(Connection::in_point)
            .collect();

        if in_points.is_empty() {
            return Err(Errors::OutPortNotConnected(out_point));
        }

        self.part.queues.send(in_points, package)
    }
}
