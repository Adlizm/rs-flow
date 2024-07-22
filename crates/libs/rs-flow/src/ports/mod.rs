
mod port;

pub use port::{Port, PortId};

///
/// Set of ports, can represent all inputs or outputs of a component
#[derive(Debug)]
pub struct Ports(pub(crate) Vec<Port>);

impl Ports {
    /// Create a new Ports
    /// 
    /// # Panics
    /// 
    /// Panic if found a Port if with same PortId or same Label
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

    /// Create a empty Ports 
    pub fn empty() -> Self {
        Ports(vec![])
    }

    /// Return if Ports is empty
    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }
    
    /// Return if exist a Port with a PortId
    pub fn contains(&self, port: PortId) -> bool {
        self.0.iter().any(|p| p.port == port)
    }
    
    /// Return if exist a Port with a label
    pub fn contains_label(&self, label: &str) -> bool {
        self.0.iter().any(|p| p.label.is_some_and(|l| l == label))
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