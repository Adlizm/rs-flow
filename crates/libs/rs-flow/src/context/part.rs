use std::fmt::Debug;

use crate::connection::Connection;

use super::{queues::Queues, global::Global};

pub(crate) struct ContextPart<G: Send + Sync > {
    pub(crate) connections: Vec<Connection>,
    pub(crate) queues: Queues,
    pub(crate) global: Global<G>
}

impl<G> Debug for ContextPart<G>
    where G: Send + Sync
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextPart")
            .finish()
    }
}

impl<G> ContextPart<G> 
    where G: Send + Sync + 'static
{
    pub(crate) fn from(connections: &Vec<Connection>, global: G) -> Self {
        Self {
            connections: connections.to_vec(),
            queues: Queues::from_connections(connections),
            global: Global::from_data(global)
        }
    }
}