use std::{collections::{HashMap, VecDeque}, sync::Mutex};

use crate::{
    connection::{Connection, Point}, 
    errors::{Errors, Result}, 
    package::Package
};


pub(crate) struct Queues {
    data: HashMap<Point, Mutex<VecDeque<Package>>>
}

impl Queues {
    pub(crate) fn from_connections(connections: &Vec<Connection>) -> Self {
        Self {
            data: connections.iter().map(|conn| {
                (Connection::in_point(conn), Mutex::new(VecDeque::new()))
            })
            .collect()
        }
    }

    pub(crate) fn receive(&self, in_point: Point) -> Result<Option<Package>> {
        if let Some(queue) = self.data.get(&in_point) {
            match queue.lock() {
                Ok(mut packages) => { Ok(packages.pop_front()) },
                Err(_) => return Err(Errors::CannotAccessQueue {
                    component: in_point.id(),
                    in_port: in_point.port(),
                }.
                into()),
            }
        } else {
            Err(Errors::InPortNotFound {
                component: in_point.id(),
                in_port: in_point.port(),
            }.into())
        }
    }

    pub(crate) fn send(&self, in_points: Vec<Point>, package: Package) -> Result<()> {
        for in_point in in_points {
            if let Some(queue) = &self.data.get(&in_point) {
                match queue.lock() {
                    Ok(mut packages) => { packages.push_back(package.clone()) },
                    
                    Err(_) => return Err(Errors::CannotAccessQueue {
                        component: in_point.id(),
                        in_port: in_point.port(),
                    }.
                    into()),
                }
            } else {
                return Err(Errors::QueueNotCreated {
                    component: in_point.id(),
                    in_port: in_point.port(),
                }
                .into());
            }
        }
        Ok(())
    }

    pub(crate) fn has_packages(&self) -> Result<Vec<Point>> {
        let mut points = Vec::new();
        for (point, queue) in &self.data {
            match queue.lock() {
                Ok(packages) => {
                    if !packages.is_empty() {
                        points.push(point.clone())
                    }
                },
                Err(_) => return Err(Errors::QueueNotCreated {
                    component: point.id(),
                    in_port: point.port(),
                }.into()),
            }
        }
        Ok(points)
    }
}
