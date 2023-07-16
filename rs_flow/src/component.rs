use crate::errors::Errors;
use crate::flow::ComponentContext;
use crate::port::{ InPort, OutPort };

pub type ComponentId = u32;

pub trait Component<T>: ComponentDefault<T> + ComponentRunnable<T> {}

pub trait ComponentDefault<T> {
    fn id(&self) -> ComponentId;
    
    fn inputs(&self) -> &Vec<InPort>;
    fn outputs(&self) -> &Vec<OutPort>;

    fn context(&self) -> Result<&ComponentContext<T>, Errors>;
    fn set_context(&mut self, context: ComponentContext<T>);
}
pub trait ComponentRunnable<T> {
    fn run(&mut self) -> Result<(), Errors>;
}