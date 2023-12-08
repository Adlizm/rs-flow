pub type PortId = u16;

#[derive(Debug, Copy, Clone)]
pub struct Port {
    pub port: PortId,
    pub label: Option<&'static str>,
    pub description: Option<&'static str>,
}

impl Port {
    pub const fn new(port: PortId) -> Self {
        Self {
            port,
            label: None,
            description: None,
        }
    }
    pub const fn from(port: PortId, label: &'static str, description: &'static str) -> Self {
        Self {
            port,
            label: Some(label),
            description: Some(description),
        }
    }
}
