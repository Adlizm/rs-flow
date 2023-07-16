use crate::errors::Errors;
use crate::flow::{ComponentContext};
use crate::package::Package;

pub type PortId = u16;

#[derive(Copy, Clone)]
pub struct InPort {
    port: PortId
}
impl InPort {
    pub fn new(port: PortId) -> Self {
        Self{ port }
    }
    pub fn port(&self) -> PortId { self.port }
    pub fn recieve<T>(&self, context: &mut ComponentContext<T>) -> Result<Package, Errors> {
        context.receive(*self)
    }
}

#[derive(Copy, Clone)]
pub struct OutPort {
    port: PortId
}
impl OutPort {
    pub fn new(port: PortId) -> Self {
        Self{ port }
    }
    pub fn port(&self) -> PortId { self.port }
    pub fn send<T>(&self, context: &mut ComponentContext<T>, package: Package) -> Result<(), Errors> {
        context.send(*self, package)
    }
}