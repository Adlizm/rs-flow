use serde::Serialize;
use serde_json::Value;
use std::marker::PhantomData;

use crate::context::Ctx;
use crate::errors::Errors;
use crate::port::{InPort, OutPort};

pub type Id = usize;

pub trait ComponentHandler {
    fn id(&self) -> Id;
    fn inputs(&self) -> Vec<InPort>;
    fn outputs(&self) -> Vec<OutPort>;
    fn set_contex(&mut self, ctx: Ctx);
    fn get_contex(&self) -> Option<&Ctx>;
    fn run(&self, ctx: &Ctx) -> Result<(), Errors>;
}
pub trait BaseComponent: Sized {
    const INPUTS: &'static [InPort];
    const OUTPUTS: &'static [OutPort];

    fn run(data: &Value, ctx: &Ctx) -> Result<(), Errors>;
}

pub struct Component<T> {
    id: Id,
    data: Value,
    context: Option<Ctx>,
    ty: PhantomData<T>,
}

impl<T> Component<T> {
    pub fn new(id: Id, data: Value) -> Self {
        Self {
            id,
            data,
            context: None,
            ty: PhantomData,
        }
    }
    pub fn from<S: Serialize>(id: Id, data: S) -> Result<Self, serde_json::Error> {
        let c = Self {
            id,
            data: serde_json::to_value(data)?,
            context: None,
            ty: PhantomData::<T>,
        };
        Ok(c)
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
    fn set_contex(&mut self, ctx: Ctx) {
        self.context = Some(ctx);
    }
    fn get_contex(&self) -> Option<&Ctx> {
        self.context.as_ref()
    }
    fn run(&self, ctx: &Ctx) -> Result<(), Errors> {
        T::run(&self.data, ctx)
    }
}

mod test {
    use crate::prelude::*;

    struct Test;

    impl BaseComponent for Test {
        const INPUTS: &'static [InPort] = &[];
        const OUTPUTS: &'static [OutPort] = &[];

        fn run(_data: &serde_json::Value, _ctxx: &Ctx) -> Result<(), Errors> {
            todo!()
        }
    }

    fn _main() {
        let _component = Component::<Test>::new(0, "testing".into());
    }
}
