use std::error;

use thiserror::Error;

use crate::component::Id;
use crate::connection::Connection;
use crate::ports::PortId;


pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync>>;


#[derive(Debug, Error)]
pub enum Errors {
    #[error("Component with id = {id:?} already exist")]
    ComponentAlreadyExist { id: Id },

    #[error("Not found a component with id = {id:?}")]
    ComponentNotFound { id: Id },

    #[error("Connection = {connection:?} already exist")]
    ConnectionAlreadyExist{ connection: Connection },

    #[error("A Loop is created with the connection = {connection:?}")]
    LoopCreated { connection: Connection },

    #[error("Component with id = {component:?} not have a Input = {in_port:?}")]
    InPortNotFound { component: Id, in_port: PortId },

    #[error("Component with id = {component:?} not have a Output = {out_port:?}")]
    OutPortNotFound { component: Id, out_port: PortId },

    #[error("A queue of componenet id = {component:?} and port = {port:?} has not created, verify if a connection with this port exist")]
    QueueNotCreated { component: Id, port: PortId },

    #[error("No packages were consumed from the component = {component:?}")]
    AnyPackageConsumed { component: Id },

    #[error("The global data could not be accessed")]
    CannotAccessGlobal,
}