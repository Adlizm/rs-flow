use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::context::CtxAsync;
use crate::errors::Result;
use crate::port::Port;


pub type Id = usize;

#[async_trait]
pub trait ComponentHandler: Send + Sync {
    type Global: Send + Sync;

    fn id(&self) -> Id;
    fn inputs(&self) -> Vec<Port>;
    fn outputs(&self) -> Vec<Port>;
    async fn run(&self, ctx: &CtxAsync<Self::Global>) -> Result<()>;
}

#[async_trait]
pub trait BaseComponent: Send + Sync {
    type Global: Send + Sync;

    const INPUTS: &'static [Port];
    const OUTPUTS: &'static [Port];
    const DESCRIPTION: &'static str;

    async fn run(&self, ctx: &CtxAsync<Self::Global>) -> Result<()>;
}
pub struct Component<T> {
    id: Id,
    data: T,
}

impl<T> Component<T> {
    pub fn new(id: Id, data: T) -> Self {
        Self { id, data }
    }
}
impl<T> Component<T>
where
    T: for<'a> Deserialize<'a>,
{
    pub fn from(id: Id, data: Value) -> Result<Self> {
        Ok(Self {
            id,
            data: serde_json::from_value(data)?,
        })
    }
}

#[async_trait]
impl<CD> ComponentHandler for Component<CD>
where
    CD: BaseComponent + Sync + Send,
{
    type Global = CD::Global;

    fn id(&self) -> Id {
        self.id
    }
    fn inputs(&self) -> Vec<Port> {
        CD::INPUTS.to_vec()
    }
    fn outputs(&self) -> Vec<Port> {
        CD::OUTPUTS.to_vec()
    }
    async fn run(&self, ctx: &CtxAsync<Self::Global>) -> Result<()> {
        self.data.run(ctx).await
    }
}
