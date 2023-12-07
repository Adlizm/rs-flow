use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use crate::prelude::*;

pub trait Queues {
    fn from(connections: &Vec<Connection>) -> Self;

    fn receive(&self, in_point: Point) -> Result<Package, Errors>;
    fn send(&self, in_points: Vec<Point>, package: Package) -> Result<(), Errors>;

    fn has_packages(&self) -> Result<Vec<Point>, Errors>;
}

pub struct AsyncQueues(HashMap<Point, Mutex<VecDeque<Package>>>);

impl Queues for AsyncQueues {
    fn from(connections: &Vec<Connection>) -> Self {
        let mut queues = HashMap::new();
        for in_point in connections.iter().map(Connection::in_point) {
            if !queues.contains_key(&in_point) {
                queues.insert(in_point.clone(), Mutex::new(VecDeque::new()));
            }
        }

        Self(queues)
    }

    fn receive(&self, in_point: Point) -> Result<Package, Errors> {
        if let Some(queue) = self.0.get(&in_point) {
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

    fn send(&self, in_points: Vec<Point>, package: Package) -> Result<(), Errors> {
        for in_point in in_points {
            if let Some(queue) = self.0.get(&in_point) {
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
        Ok(())
    }

    fn has_packages(&self) -> Result<Vec<Point>, Errors> {
        let mut points = Vec::new();
        for (point, queue) in self.0.iter() {
            match queue.lock() {
                Ok(queue) => {
                    if !queue.is_empty() {
                        points.push(point.clone());
                    }
                }
                Err(_) => return Err(Errors::CannotAccessQueue(point.clone())),
            }
        }
        Ok(points)
    }
}
