use rs_flow::prelude::*;
use rs_flow_macros::Component;
use crate::components::MyGlobal;

#[derive(Component)]
#[inputs  [input1] ]
pub struct Log {}
impl ComponentRunnable<MyGlobal> for LogComponent<MyGlobal> {
    fn run(&mut self) -> Result<(), Errors> {
        let package = self.context()?.receive(self.inputs[0])?;
        let global = self.context()?.global()?;

        println!("{:#}", package.content());
        println!("{}", global.global_message);
        Ok(())
    }
}
impl Component<MyGlobal> for LogComponent<MyGlobal> {}
