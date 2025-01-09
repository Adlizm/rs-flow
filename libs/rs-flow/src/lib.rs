#![feature(map_many_mut)]

mod flow;
pub use flow::Flow;

mod error;
pub use error::{Error, RunResult as Result};

mod context;
pub use context::Ctx;

mod package;
pub use package::Package;

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
    pub use crate::package::Package;
    pub use crate::ports::*;

    pub use crate::error::{Error, RunResult as Result};
    pub use crate::Ctx;
    pub use async_trait::async_trait;
}
