use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use crate::{
    package::Package,
    connection::{Connection, Point},
    errors::{Result, Errors}
};

pub trait Queues {
    fn from_connections(connections: &Vec<Connection>) -> Self;

    fn receive(&self, in_point: Point) -> Result<Package>;
    fn send(&self, in_points: Vec<Point>, package: Package) -> Result<()>;

    fn has_packages(&self) -> Result<Vec<Point>>;
}

pub struct AsyncQueues(HashMap<Point, Mutex<VecDeque<Package>>>);

impl Queues for AsyncQueues {
    fn from_connections(connections: &Vec<Connection>) -> Self {
        let mut queues = HashMap::new();
        for in_point in connections.iter().map(Connection::in_point) {
            if !queues.contains_key(&in_point) {
                queues.insert(in_point.clone(), Mutex::new(VecDeque::new()));
            }
        }

        Self(queues)
    }

    fn receive(&self, in_point: Point) -> Result<Package> {
        if let Some(queue) = self.0.get(&in_point) {
            return match queue.lock() {
                Ok(mut queue) => {
                    if let Some(package) = queue.pop_front() {
                        Ok(package)
                    } else {
                        Err(Errors::EmptyQueue {
                            component: in_point.id(),
                            in_port: in_point.port(),
                        }
                        .into())
                    }
                }
                Err(_) => Err(Errors::CannotRecievePackage.into()),
            };
        } else {
            Err(Errors::InPortNotFound {
                component: in_point.id(),
                in_port: in_point.port(),
            }
            .into())
        }
    }

    fn send(&self, in_points: Vec<Point>, package: Package) -> Result<()> {
        for in_point in in_points {
            if let Some(queue) = self.0.get(&in_point) {
                match queue.lock() {
                    Ok(mut packages) => {
                        packages.push_back(package.clone());
                    }
                    Err(_) => {
                        return Err(Errors::CannotAccessQueue {
                            component: in_point.id(),
                            in_port: in_point.port(),
                        }
                        .into());
                    }
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

    fn has_packages(&self) -> Result<Vec<Point>> {
        let mut points = Vec::new();
        for (point, queue) in self.0.iter() {
            match queue.lock() {
                Ok(queue) => {
                    if !queue.is_empty() {
                        points.push(point.clone());
                    }
                }
                Err(_) => {
                    return Err(Errors::CannotAccessQueue {
                        component: point.id(),
                        in_port: point.port(),
                    }
                    .into())
                }
            }
        }
        Ok(points)
    }
}
