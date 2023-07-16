use rs_flow::prelude::*;

use main::components::message::{Message, MessageComponent};
use main::components::log::{Log, LogComponent};
use main::components::MyGlobal;

fn crate_flow_and_run() -> Result<(), Errors> {
    let data = Message{ message: "Hello World".to_owned() };
    let mut message = MessageComponent::<MyGlobal>::new(1, data);

    let data = Log{  };
    let mut log = LogComponent::<MyGlobal>::new(2, data);

    let mut flow = Flow::new();
    flow.add_component(&mut message)?
        .add_component(&mut log)?
        .add_connection(Connection::new(1, 0, 2, 0))?
        .build()?;

    flow.run(MyGlobal{ global_message: "My Global message".to_owned() })?;
    return Ok(());
}
fn main() {
    crate_flow_and_run().unwrap();
}
