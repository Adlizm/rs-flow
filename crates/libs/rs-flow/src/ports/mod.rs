
mod port;

pub use port::{Port, PortId};

pub struct Ports(Vec<Port>);

impl Ports {
    pub fn empty() -> Self {
        Ports(vec![])
    }
    pub fn new(ports: Vec<Port>) -> Self {
        let length = ports.len();
        let mut i = 0;
        while i < length {
            let mut j = i + 1;
            while j < length {
                if ports[i].port == ports[j].port {
                    panic!("Found ports with same id")
                }
                if ports[i].label.is_some() && ports[i].label == ports[j].label {
                    panic!("Found ports with same label")
                }
                j += 1;
            }
            i += 1;
        }
        Self(ports)
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }
    pub fn contains(&self, port: PortId) -> bool {
        self.0.iter().any(|p| p.port == port)
    }
    pub fn contains_label(&self, label: &str) -> bool {
        self.0.iter().any(|p| p.label.is_some_and(|l| l == label))
    }

    pub(crate) fn all(&self, f: impl FnMut(&Port) -> bool) -> bool {
        self.0.iter().all(f)
    }
}


pub trait Inputs {
    fn inputs(&self) -> &Ports;
    fn input(&self, label: &'static str) -> PortId;
}
pub trait Outputs {
    fn outputs(&self) -> &Ports;
    fn output(&self, label: &'static str) -> PortId;
}