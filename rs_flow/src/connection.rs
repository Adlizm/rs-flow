use crate::port::PortId;
use crate::component::ComponentId;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Connection {
    pub from: ComponentId,
    pub out_port: PortId,
    pub to: ComponentId,
    pub in_port: PortId,
}


pub type InPoint = (ComponentId, PortId);
pub type OutPoint = (ComponentId, PortId);


impl Connection {
    pub fn new(from: ComponentId, out_port: PortId, to: ComponentId, in_port: PortId) -> Self {
        Self { from, out_port, to, in_port }
    }

    
}