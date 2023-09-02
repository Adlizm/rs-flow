use std::error::Error;
use std::fmt::Display;

use crate::component::Id;
use crate::connection::{Connection, Point};

#[derive(Debug, Clone)]
pub enum Errors {
    ComponentAlreadyExist { id: Id },
    ComponentNotFound { id: Id },
    InPortNotFound(Point),
    OutPortNotFound(Point),
    ConnectionAlreadyExist(Connection),

    OutPortNotConnected(Point),

    CannotAccessQueue(Point),
    CannotResetQueue(Point),
    QueueNotCreated(Point),
    EmptyQueue(Point),

    ContextNotLoaded,
    CannotLoadGlobal,
    CannotSendPackage,
    CannotRecievePackage,
    CannotReadState,
    CannotUpdateState,

    FlowNotRunning,
    FlowUnreadyToRun,
    FlowAlreadyBuilded,
    FlowAlreadyRunning,

    NotImplemented,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl Error for Errors {}
