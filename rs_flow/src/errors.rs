use crate::connection::Connection;
use crate::component::ComponentId;
use crate::port::PortId;

#[derive(Debug)]
pub enum Errors {
    ComponentAlreadyExist(ComponentId),
    ComponentNotFound(ComponentId),
    InPortNotFound(ComponentId, PortId),
    OutPortNotFound(ComponentId, PortId),
    ConnectionAlreadyExist(Connection),
    
    OutPortNotConnected(ComponentId, PortId),
    
    CannotAccessQueue(ComponentId, PortId),
    CannotResetQueue(ComponentId, PortId),
    QueueNotCreated(ComponentId, PortId),
    EmptyQueue(ComponentId, PortId),

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

    NotImplemented
}
