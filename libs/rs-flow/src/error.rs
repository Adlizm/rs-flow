use crate::component::Id;
use crate::connection::Connection;
use crate::ports::PortId;

pub type Result<T> = std::result::Result<T, Error>;
pub type RunResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Component with id = {id:?} already exist")]
    ComponentAlreadyExist { id: Id },

    #[error("Not found a operator with id = {id:?}")]
    ComponentNotFound { id: Id },

    #[error("Connection = {connection:?} already exist")]
    ConnectionAlreadyExist { connection: Connection },

    #[error("A Loop is created with the connection = {connection:?}")]
    LoopCreated { connection: Connection },

    #[error("Component with id = {component:?} not have a Input = {in_port:?}")]
    InPortNotFound { component: Id, in_port: PortId },

    #[error("Component with id = {component:?} not have a Output = {out_port:?}")]
    OutPortNotFound { component: Id, out_port: PortId },

    #[error("Try recive on component id = {component:?} in ports = {ports:?}, buts ports not exist or repeted")]
    InvalidMultipleRecivedPorts { component: Id, ports: Vec<PortId> },

    #[error("No packages were consumed from the component = {component:?}")]
    AnyPackageConsumed { component: Id },

    #[error("The global data could not be accessed")]
    CannotAccessGlobal,
}
