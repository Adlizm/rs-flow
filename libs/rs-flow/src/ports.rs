pub type PortId = u16;

///
/// One of the [Ports] of a [Component](crate::component::Component)
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Port {
    /// [Port] id, indentify a Input/Outpot [Port] of a [Component](crate::component::Component)
    pub port: PortId,

    /// A Name given for this port. When use #[derive(Inputs, Outputs)] that label is created as the type name.
    pub label: Option<&'static str>,

    /// Description of what mean a [Package](crate::package::Package) send/recieve by this [Port]
    pub description: Option<&'static str>,
}

impl Port {
    /// Create a [Port] with that [PortId], tha label and description is [None].
    pub const fn new(port: PortId) -> Self {
        Self {
            port,
            label: None,
            description: None,
        }
    }
    /// Define a [Port] with all.
    pub const fn from(
        port: PortId,
        label: &'static str,
        description: Option<&'static str>,
    ) -> Self {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Ports(&'static [Port]);

impl Ports {
    /// Create a new Ports
    ///
    /// # Panics
    ///
    /// Panic if found two [Port]'s if with same [PortId] or same label
    ///
    pub const fn new(ports: &'static [Port]) -> Self {
        let length = ports.len();
        let mut i = 0;
        while i < length {
            let mut j = i + 1;
            while j < length {
                if ports[i].port == ports[j].port {
                    panic!("Found ports with same id")
                }
                j += 1;
            }
            i += 1;
        }
        Self(ports)
    }

    /// Create a empty Ports
    pub fn empty() -> Self {
        Self(&[])
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

    /// Return a Iterator foreach port
    pub fn iter(&self) -> impl Iterator<Item = &Port> {
        self.0.iter()
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
/// #[derive(Inputs)]
/// struct Bytes;
///
/// struct WriteFile {
///     filepath: String
/// }
///
/// #[async_trait]
/// impl ComponentSchema for WriteFile {
///     type Inputs = Bytes;
///     type Outputs = ();
///     type Package = Vec<u8>;
///
///     async fn run(&self, ctx: &mut Ctx<Self>) -> Result<Next> {
///         if let Some(bytes) = ctx.receive(Bytes) {
///             std::fs::write(&self.filepath, &bytes)?;
///         }
///         Ok(Next::Continue)
///     }
///
/// }
///
/// ```
///
/// In this exemple, `Bytes` implement the [Inputs] trait a [Port],
/// gives a meaning that each [Package](crate::package::Package) received by it,
/// must contain the bytes to write in the file created.
///
///
pub trait Inputs {
    /// All outputs [Ports] of a [Component](crate::component::Component)
    const PORTS: Ports;

    /// Return a input [PortId] of a [Component](crate::component::Component)
    fn into_port(&self) -> PortId;
}

impl Inputs for () {
    const PORTS: Ports = Ports(&[]);

    fn into_port(&self) -> PortId {
        panic!("Component not have a input port");
    }
}

///
/// Define all outputs [Port] of a [Component](crate::component::Component).
/// Each of this [Port] represent a way to send a [Package](crate::package::Package)
/// to other [Component](crate::component::Component)
///
/// ```
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use std::any::Any;
///
/// use rs_flow::prelude::*;
///
/// #[derive(Outputs)]
/// enum Out {
///     #[description("Environment Variables loaded by .env file")]
///     Env,
///     Error
/// }
///
/// struct LoadEnv;
///
/// #[async_trait]
/// impl ComponentSchema for LoadEnv {
///     type Outputs = Out;
///     type Inputs = ();
///     type Package = Arc<dyn Any + Send + Sync>;
///
///     async fn run(&self, ctx: &mut Ctx<Self>) -> Result<Next> {
///         match load_envs() {
///             Ok(envs) => ctx.send(Out::Env, Arc::new(envs)),
///             Err(error) => ctx.send(Out::Error, Arc::new(error))
///         }
///         Ok(Next::Continue)
///     }
/// }
///
/// fn load_envs() -> std::result::Result<HashMap<String, String>, String> {
///     // load env vars from a file .env em create a packge for them
///     todo!()
/// }
/// ```
///
/// In this exemple, `Out` implement the [Outputs] trait and have two [Port]'s,
/// each [Port] gives a meaning to each [Package](crate::package::Package) send by
/// `LoadEnv` [Component](crate::component::ComponentSchema).
///
///
/// For example:
///     <code> ctx.send(Out::Error, Package::empty()) </code>
///
/// `LoadEnv` send a [Package](crate::package::Package) that shows to the
/// [Component](crate::component::Component) receiving this package that the environment
/// variables were not being loaded, and handle this in some way.
///
pub trait Outputs {
    /// All outputs [Ports] of a [Component](crate::component::Component)
    const PORTS: Ports;

    /// Return a output [PortId] of a [Component](crate::component::Component)
    fn into_port(&self) -> PortId;
}

impl Outputs for () {
    const PORTS: Ports = Ports(&[]);

    fn into_port(&self) -> PortId {
        panic!("Component not have a output port");
    }
}
