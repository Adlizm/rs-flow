use serde::Serialize;

pub type PortId = u16;

#[derive(Debug, Clone, Serialize)]
pub struct Port {
    pub port: PortId,
    pub label: Option<&'static str>,
    pub description: Option<&'static str>,
}

impl Port {
    pub fn new(port: PortId) -> Self {
        Self {
            port,
            label: None,
            description: None,
        }
    }
    pub fn from(port: PortId, label: &'static str, description: Option<&'static str>) -> Self {
        Self {
            port,
            label: Some(label),
            description: description,
        }
    }
}
