mod flow;
pub use flow::Flow;

mod error;
pub use error::{FlowError, RunResult as Result};

mod context;
pub use context::Ctx;


pub mod component;
pub mod connection;
pub mod package;
pub mod ports;

pub mod macros {
    pub use rs_flow_macros::{inputs, outputs};
}

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::component::*;
    pub use crate::connection::Connection;
    pub use crate::flow::Flow;
    pub use crate::package::Package;
    pub use crate::ports::*;

    pub use crate::Ctx;
    pub use crate::error::{FlowError, RunResult as Result};
    pub use async_trait::async_trait;
    
}



