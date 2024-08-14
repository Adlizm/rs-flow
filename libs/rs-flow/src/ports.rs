use serde::Serialize;

pub type PortId = u16;

/// 
/// One of the [Ports](super::Ports) of a [Component](crate::component::Component)
/// 
#[derive(Debug, Clone, Serialize)]
pub struct Port {
    /// [Port] id, indentify a Input/Outpot [Port] of a [Component](crate::component::Component)
    pub port: PortId,
    
    /// A other way to identify this [Port], can be constructed with the [inputs](crate::macros::inputs)/[outputs](crate::macros::outputs) macro
    pub label: Option<&'static str>,

    /// Description of what mean a [Package](crate::package::Package) send/recieve by this [Port]
    pub description: Option<&'static str>,
}

impl Port {
    /// Create a [Port] with that [PortId], tha label and description is [None].
    pub fn new(port: PortId) -> Self {
        Self {
            port,
            label: None,
            description: None,
        }
    }
    /// Define a [Port] with all.
    pub fn from(port: PortId, label: &'static str, description: Option<&'static str>) -> Self {
        Self {
            port,
            label: Some(label),
            description,
        }
    }
}

///
/// Set of [Port]'s, can represent all [Inputs] or [Outputs] of a [Component](crate::component::Component)
/// 
#[derive(Debug)]
pub struct Ports(pub(crate) Vec<Port>);

impl Ports {
    /// Create a new Ports
    /// 
    /// # Panics
    /// 
    /// Panic if found two [Port]'s if with same [PortId] or same label
    /// 
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

///
/// Define all inputs [Port] of a [Component](crate::component::Component).
/// Each of this [Port] represent a way to receive a [Package](crate::package::Package) 
/// from other [Component](crate::component::Component)
/// 
/// ```
/// use rs_flow::prelude::*;
/// 
/// #[inputs { 
///     url: { description = "Url to send the Request" }, 
///     method: { description = "Http Method (GET, POST, etc)" }, 
///     body: { description = "Body from Request" }
/// }]
/// struct SendRequest;
/// ```
/// 
/// In this exemple, `SendRequest` implement the [Inputs] trait and have 3 [Port]'s,
/// each [Port] gives a meaning to each [Package](crate::package::Package) received by it.
/// 
/// 
/// For example: <code> ctx.receive(self.input("body")) </code> 
/// Recieve a [Package](crate::package::Package)'s that contains the Body of the HTTP Request.
/// 
pub trait Inputs {
    fn inputs(&self) -> &Ports;
    fn input(&self, label: &'static str) -> PortId;
}

///
/// Define all outputs [Port] of a [Component](crate::component::Component).
/// Each of this [Port] represent a way to send a [Package](crate::package::Package) 
/// to other [Component](crate::component::Component)
/// 
/// ```
/// use rs_flow::prelude::*;
/// 
/// #[outputs { 
///     envs: { description = "Environment Variables loaded by .env file" },
///     error
/// }]
/// struct LoadEnvs;
/// ```
/// 
/// In this exemple, `LoadEnvs` implement the [Outputs] trait and have two [Port]'s,
/// each [Port] gives a meaning to each [Package](crate::package::Package) send by it.
/// 
/// 
/// For example: <code> ctx.send(self.output("error"), Package::empty()) </code>  
/// Send a [Package](crate::package::Package) that shows to the [Component](crate::component::Component) 
/// receiving this package that the environment variables were not being loaded, 
/// and handle this in some way.
/// 
pub trait Outputs {
    /// All outputs [Ports] of a [Component](crate::component::Component)
    fn outputs(&self) -> &Ports;

    /// Return a output [PortId] of a [Component](crate::component::Component) by the label
    fn output(&self, label: &'static str) -> PortId;
}