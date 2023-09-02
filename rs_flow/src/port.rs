use crate::context::Ctx;
use crate::errors::Errors;
use crate::package::Package;

pub type PortId = u16;

#[derive(Copy, Clone)]
pub struct InPort {
    pub port: PortId,
}
impl InPort {
    pub fn new(port: PortId) -> Self {
        Self { port }
    }
    pub fn port(&self) -> PortId {
        self.port
    }
    pub fn recieve(&self, context: &mut Ctx) -> Result<Package, Errors> {
        context.receive(*self)
    }
}

#[derive(Copy, Clone)]
pub struct OutPort {
    pub port: PortId,
}
impl OutPort {
    pub fn new(port: PortId) -> Self {
        Self { port }
    }
    pub fn port(&self) -> PortId {
        self.port
    }
    pub fn send<T>(&self, context: &mut Ctx, package: Package) -> Result<(), Errors> {
        context.send(*self, package)
    }
}
