use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::context::Ctx;
use crate::errors::Result;
use crate::ports::{Inputs, Outputs};


pub type Id = usize;
#[async_trait]
pub trait ComponentRunnable: Send + Sync + Inputs + Outputs + 'static {
    type Global: Send + Sync;

    async fn run(&self, ctx: Ctx<Self::Global>) -> Result<()>;
}


pub struct Component<G> {
    pub(crate) id: Id,
    pub(crate) data: Box<dyn ComponentRunnable<Global = G>>,
}

impl<G> Component<G> {
    pub fn new<T>(id: Id, data: T) -> Self 
        where T: ComponentRunnable<Global = G>
    {
        Self { id, data: Box::new(data) }
    }

    pub fn from<T>(id: Id, data: Value) -> Result<Self> 
        where T: ComponentRunnable<Global = G> + for<'d> Deserialize<'d>
    {
        let data: T = serde_json::from_value(data)?;
        Ok(Self {
            id,
            data: Box::new(data),
        })
    }
}
