use std::sync::Arc;

use crate::connection::Connection;

use super::{queues::{AsyncQueues, Queues}, global::{GlobalAsync, Global}};

pub(crate) struct ContextPartAsync<GD> {
    pub(crate) connections: Vec<Connection>,
    pub(crate) queues: AsyncQueues,
    pub(crate) global: GlobalAsync<GD>
}

impl<GD> ContextPartAsync<GD> {
    pub(crate) fn from(connections: &Vec<Connection>, global: GD) -> Arc<Self> {
        Arc::new(Self {
            connections: connections.to_vec(),
            queues: AsyncQueues::from_connections(connections),
            global: GlobalAsync::from_data(global)
        })
    }
}