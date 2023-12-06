pub type PortId = u16;

#[derive(Copy, Clone)]
pub struct Port {
    pub port: PortId,
}

impl Port {
    pub fn new(port: PortId) -> Self {
        Self { port }
    }
}
