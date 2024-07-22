mod connection;

use std::collections::HashMap;

pub use connection::Connection;
pub(crate) use connection::Point;

use crate::component::Id;
use crate::errors::{Errors, Result};

pub(crate) struct Connections {
    parents: HashMap<Id, Vec<Id>>,
    connections: HashMap<Point, Vec<Point>>
}


impl Default for Connections {
    fn default() -> Connections {
        Connections { 
            parents: Default::default(),
            connections: Default::default()
        }
    }
}

impl Connections {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, connection: Connection) -> Result<()> {
        if self.ancestral_of(connection.from, connection.to) {
            return Err(Errors::LoopCreated { connection }.into())
        }

        let entry = self.connections.entry(connection.from());
        let to = connection.to();
        let to_ports = entry.or_default();

        if to_ports.contains(&to) {
            return Err(Errors::ConnectionAlreadyExist { connection }.into())
        }
        
        to_ports.push(to);
        
        let parents = self.parents.entry(connection.to).or_default();
        if !parents.contains(&connection.from) {
            parents.push(connection.from);
        }

        Ok(())
    }

    pub fn ancestral_of(&self, ancestral: Id, id: Id) -> bool {
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
        if let Some(parents) = self.parents.get(&id) {
            for parent in parents {
                if ancestrals.contains(parent) || self.any_ancestral_of(ancestrals, *parent) {
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