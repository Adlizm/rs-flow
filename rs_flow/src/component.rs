use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::context::queues::AsyncQueues;
use crate::context::Ctx;
use crate::errors::Result;
use crate::port::Port;

pub type Id = usize;

#[async_trait]
pub trait ComponentHandler {
    fn id(&self) -> Id;
    fn inputs(&self) -> Vec<Port>;
    fn outputs(&self) -> Vec<Port>;
    async fn run(&self, ctx: &Ctx<AsyncQueues>) -> Result<()>;
}

#[async_trait]
pub trait BaseComponent: Sized {
    const INPUTS: &'static [Port];
    const OUTPUTS: &'static [Port];

    async fn run(&self, ctx: &Ctx<AsyncQueues>) -> Result<()>;
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
impl<T> ComponentHandler for Component<T>
where
    T: BaseComponent + Sync + Send,
{
    fn id(&self) -> Id {
        self.id
    }
    fn inputs(&self) -> Vec<Port> {
        T::INPUTS.to_vec()
    }
    fn outputs(&self) -> Vec<Port> {
        T::OUTPUTS.to_vec()
    }
    async fn run(&self, ctx: &Ctx<AsyncQueues>) -> Result<()> {
        self.data.run(ctx).await
    }
}

mod test {
    use async_trait::async_trait;

    use crate::{context::queues::AsyncQueues, prelude::*};

    #[derive(Default)]
    pub struct Test {
        pub message: String,
    }

    #[async_trait]
    impl BaseComponent for Test {
        const INPUTS: &'static [Port] = &[];
        const OUTPUTS: &'static [Port] = &[];

        async fn run(&self, _ctx: &Ctx<AsyncQueues>) -> Result<()> {
            println!("Message: {}", self.message);
            Ok(())
        }
    }

    fn _main() {
        let _component = Component::new(0, Test::default());
    }
}
