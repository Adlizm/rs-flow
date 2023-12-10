use std::error;
use std::fmt::Display;

use crate::component::Id;
use crate::connection::Connection;
use crate::port::PortId;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Errors {
    ComponentAlreadyExist { id: Id },
    ComponentNotFound { id: Id },

    ConnectionAlreadyExist(Connection),

    InPortNotFound { component: Id, in_port: PortId },
    OutPortNotFound { component: Id, out_port: PortId },
    OutPortNotConnected { component: Id, out_port: PortId },

    CannotAccessQueue { component: Id, in_port: PortId },
    QueueNotCreated { component: Id, in_port: PortId },
    EmptyQueue { component: Id, in_port: PortId },

    CannotSendPackage,
    CannotRecievePackage,

    CannotAccessGlobal,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl error::Error for Errors {}
