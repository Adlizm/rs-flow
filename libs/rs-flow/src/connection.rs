use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::component::Id;
use crate::ports::PortId;
use crate::error::{FlowError, Result};


///
/// A connection between two components, connecting this componets with a 
/// [Output](crate::ports::Outputs) [Port](crate::ports::Port) of a [Component](crate::component::Component) 
/// and a [Input](crate::ports::Inputs) [Port](crate::ports::Port) from the other [Component](crate::component::Component).
/// 
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Connection {
    pub from: Id,
    pub out_port: PortId,
    pub to: Id,
    pub in_port: PortId,
}


///
/// This struct can represent a [Port](crate::ports::Port) of [`Input`](crate::ports::Inputs)/[`Output`](crate::ports::Outputs)  of a component.
/// Two of this [Point] can represent a [Connection] that connect two components
/// 
/// ```
/// use rs_flow::connection::{Point, Connection};
/// 
/// let from = Point::new(1, 0);
/// let to = Point::new(2, 1);
/// 
/// let conn = Connection::by(from, to);
/// 
/// assert_eq!(conn.from(), from);
/// assert_eq!(conn.to(), to);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    id: Id,
    port: PortId,
}
impl Point {
    /// Create a new Point
    #[inline]
    pub const fn new(id: Id, port: PortId) -> Self {
        Self { id, port }
    }

    /// Id of component representation
    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    /// PortId of component representation port
    #[inline]
    pub fn port(&self) -> PortId {
        self.port
    }
}


impl From<(Id, PortId)> for Point {
    #[inline]
    fn from((id, port): (Id, PortId)) -> Self {
        Point { id, port }
    }
}

impl Connection {
    /// Create a new connection
    #[inline]
    pub const fn new(from: Id, out_port: PortId, to: Id, in_port: PortId) -> Self {
        Self {
            from,
            out_port,
            to,
            in_port,
        }
    }

    /// Create a connection by two Points
    #[inline]
    pub const fn by(from: Point, to: Point) -> Self {
        Self { 
            from: from.id, 
            out_port: from.port, 
            to: to.id, 
            in_port: to.port 
        }
    }

    /// Return from Point of this connection 
    #[inline]
    pub fn from(&self) -> Point {
        Point::new(self.from, self.out_port)
    }

    /// Return to Point of this connection 
    #[inline]
    pub fn to(&self) -> Point {
        Point::new(self.to, self.in_port)
    }
}


/// 
/// Graph of Flow connections.
///  
/// This struct provide a rapid access to calculate ancestrals of a component
/// that is usefull for know when components of [`Eager`](crate::component::Type#variant.Eager) type is ready to run.
/// 
/// That graph cannot create a Loop, end return a error if try
/// add a connection that create a Loop.
/// 
#[derive(Debug, Clone)]
pub(crate) struct Connections {
    parents: HashMap<Id, Vec<Id>>,
    connections: HashMap<Point, Vec<Point>>
}


/// Empty graph of Flow connections
impl Default for Connections {
    fn default() -> Connections {
        Connections { 
            parents: Default::default(),
            connections: Default::default()
        }
    }
}

impl Connections {
    /// Create a empty connections graph 
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a connection 
    pub fn add(&mut self, connection: Connection) -> Result<()> {
        if self.ancestral_of(connection.from, connection.to) {
            return Err(FlowError::LoopCreated { connection }.into())
        }

        let entry = self.connections.entry(connection.from());
        let to = connection.to();
        let to_ports = entry.or_default();

        if to_ports.contains(&to) {
            return Err(FlowError::ConnectionAlreadyExist { connection }.into())
        }
        
        to_ports.push(to);
        
        let parents = self.parents.entry(connection.to).or_default();
        if !parents.contains(&connection.from) {
            parents.push(connection.from);
        }

        Ok(())
    }

    
    pub fn ancestral_of(&self, ancestral: Id, id: Id) -> bool {
        if ancestral == id { // prevent component that connect to your self
            return true;
        }
        if let Some(parents) = self.parents.get(&id) {
            for parent in parents {
                if *parent == ancestral || self.ancestral_of(ancestral, *parent) {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn any_ancestral_of(&self, ancestrals: &[Id], id: Id) -> bool {
        if ancestrals.contains(&id) { // prevent component that connect to your self
            return true;
        }

        if let Some(parents) = self.parents.get(&id) {
            for parent in parents {
                if self.any_ancestral_of(ancestrals, *parent) {
                    return true;
                }
            }
        }

        false
    }

    pub fn from(&self, from: Point) -> Option<&Vec<Point>> {
        self.connections.get(&from)
    }
}