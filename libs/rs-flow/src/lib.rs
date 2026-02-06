mod flow;
pub use flow::Flow;

mod error;
pub use error::{Error, RunResult as Result};

pub mod context;

/// Structs for component infos and the trait [ComponentSchema](crate::component::ComponentSchema)
pub mod component;
/// Structs for connect two components and their ports in a [Flow]
pub mod connection;
/// Structs for ports of components and the traits [Inputs](crate::ports::Inputs) and [Outputs](crate::ports::Outputs)
pub mod ports;

/// Macros for derive [Inputs](crate::ports::Inputs) and [Outputs](crate::ports::Outputs) trait
pub mod macros {
    pub use rs_flow_macros::{Inputs, Outputs};
}

/// Common imports for use `rs_flow` crate
pub mod prelude {
    pub use crate::component::*;
    pub use crate::connection::Connection;
    pub use crate::flow::Flow;
    pub use crate::macros::*;
    pub use crate::ports::*;

    pub use crate::context::{Ctx, Global};
    pub use crate::error::{Error, RunResult as Result};
    pub use async_trait::async_trait;
}
