use rs_flow::prelude::*;
use rs_flow_macros::Component;
use serde::Serialize;
use crate::components::MyGlobal;

#[derive(Serialize)]
#[derive(Component)]
#[outputs [output1] ]
pub struct Message {
    pub message: String
}
impl ComponentRunnable<MyGlobal> for MessageComponent<MyGlobal> {
    fn run(&mut self) -> Result<(), Errors> {
        let package = Package::new(&self.data);

        self.context()?.send(self.outputs[0], package)?;
        Ok(())
    }
}
impl Component<MyGlobal> for MessageComponent<MyGlobal> {}
