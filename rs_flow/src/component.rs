use serde::Deserialize;
use serde_json::Value;

use crate::context::Ctx;
use crate::errors::Errors;
use crate::port::{InPort, OutPort};

pub type Id = usize;

pub trait ComponentHandler {
    fn id(&self) -> Id;
    fn inputs(&self) -> Vec<InPort>;
    fn outputs(&self) -> Vec<OutPort>;
    fn run(&self, ctx: &Ctx) -> Result<(), Errors>;
}
pub trait BaseComponent: Sized {
    const INPUTS: &'static [InPort];
    const OUTPUTS: &'static [OutPort];

    fn run(&self, ctx: &Ctx) -> Result<(), Errors>;
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
    pub fn from(id: Id, data: Value) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id,
            data: serde_json::from_value(data)?,
        })
    }
}

impl<T> ComponentHandler for Component<T>
where
    T: BaseComponent,
{
    fn id(&self) -> Id {
        self.id
    }
    fn inputs(&self) -> Vec<InPort> {
        T::INPUTS.to_vec()
    }
    fn outputs(&self) -> Vec<OutPort> {
        T::OUTPUTS.to_vec()
    }
    fn run(&self, ctx: &Ctx) -> Result<(), Errors> {
        self.data.run(ctx)
    }
}

mod test {
    use crate::prelude::*;

    #[derive(Default)]
    pub struct Test {
        pub message: String,
    }

    impl BaseComponent for Test {
        const INPUTS: &'static [InPort] = &[];
        const OUTPUTS: &'static [OutPort] = &[];

        fn run(&self, _ctx: &Ctx) -> Result<(), Errors> {
            println!("Message: {}", self.message);
            Ok(())
        }
    }

    fn _main() {
        let _component = Component::new(0, Test::default());
    }
}
