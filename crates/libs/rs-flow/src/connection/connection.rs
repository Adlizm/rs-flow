use serde::{Serialize, Deserialize};

use crate::component::Id;
use crate::ports::PortId;

///
/// Create a connection between two components.
/// Connecting a output port of a component with a input port of other component.
/// 
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Connection {
    pub from: Id,
    pub out_port: PortId,
    pub to: Id,
    pub in_port: PortId,
}


///
/// This struct can represent a Input/Output port of a component.
/// Two of this Point can represent a Connection that connect two components
/// 
/// ```
/// use rs_flow::connection::{Point, Connection};
/// 
/// let from = Point::new(1, 0);
/// let to = Point::new(2, 1);
/// 
/// let conn = Connection::by(from.clone(), to.clone());
/// 
/// assert_eq!(conn.from(), from);
/// assert_eq!(conn.to(), to);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    id: Id,
    port: PortId,
}
impl Point {
    /// Create a new Point
    pub const fn new(id: Id, port: PortId) -> Self {
        Self { id, port }
    }
    /// Id of component representation
    pub fn id(&self) -> Id {
        self.id
    }
    /// PortId of component representation port
    pub fn port(&self) -> PortId {
        self.port
    }
}

impl Connection {
    /// Create a new connection
    pub const fn new(from: Id, out_port: PortId, to: Id, in_port: PortId) -> Self {
        Self {
            from,
            out_port,
            to,
            in_port,
        }
    }

    /// Create a connection by two Points
    pub const fn by(from: Point, to: Point) -> Self {
        Self { 
            from: from.id, 
            out_port: from.port, 
            to: to.id, 
            in_port: to.port 
        }
    }

    /// Return from Point of this connection 
    pub fn from(&self) -> Point {
        Point::new(self.from, self.out_port)
    }

    /// Return to Point of this connection 
    pub fn to(&self) -> Point {
        Point::new(self.to, self.in_port)
    }
}
