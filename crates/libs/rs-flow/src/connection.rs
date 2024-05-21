use serde::{Serialize, Deserialize};

use crate::component::Id;
use crate::ports::PortId;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Connection {
    pub from: Id,
    pub out_port: PortId,
    pub to: Id,
    pub in_port: PortId,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    id: Id,
    port: PortId,
}
impl Point {
    pub fn new(id: Id, port: PortId) -> Self {
        Self { id, port }
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn port(&self) -> PortId {
        self.port
    }
}

impl Connection {
    pub const fn new(from: Id, out_port: PortId, to: Id, in_port: PortId) -> Self {
        Self {
            from,
            out_port,
            to,
            in_port,
        }
    }
    pub fn out_point(&self) -> Point {
        Point::new(self.from, self.out_port)
    }
    pub fn in_point(&self) -> Point {
        Point::new(self.to, self.in_port)
    }
}
