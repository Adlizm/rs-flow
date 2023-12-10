use std::sync::Arc;

use connection::Connection;

use crate::component::Id;
use crate::connection::{self, Point};
use crate::errors::{Errors, Result};
use crate::package::Package;
use crate::prelude::Port;

use self::queues::Queues;
use self::global::Global;

pub mod queues;
pub mod global;
pub mod part;

pub struct CtxAsync<GD> {
    id: Id,
    part: Arc<part::ContextPartAsync<GD>>,
}

impl<GD> CtxAsync<GD> {
    pub(crate) fn from(id: Id, part: &Arc<part::ContextPartAsync<GD>>) -> Self {
        Self {
            id,
            part: part.clone(),
        }
    }
    pub fn receive(&self, in_port: &Port) -> Result<Package> {
        let in_point = Point::new(self.id, in_port.port);
        self.part.queues.receive(in_point)
    }
    pub fn send(&self, out_port: &Port, package: Package) -> Result<()> {
        let out_point = Point::new(self.id, out_port.port);

        let in_points: Vec<Point> = self
            .part
            .connections
            .iter()
            .filter(|conn| conn.out_point() == out_point)
            .map(Connection::in_point)
            .collect();

        if in_points.is_empty() {
            return Err(Errors::OutPortNotConnected {
                component: self.id,
                out_port: out_port.port,
            }
            .into());
        }

        self.part.queues.send(in_points, package)
    }
    pub fn with_global<R>(&self,  call: impl FnOnce(&mut GD) -> R) -> Result<R> {
        self.part.global.with_global(call)
    }

}
