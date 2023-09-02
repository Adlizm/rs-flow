use rs_flow::prelude::*;

use main::components::log::Log;
use main::components::message::Message;

fn crate_flow_and_run() -> Result<(), Errors> {
    let message = Component::<Message>::new(1, "Hello World".into());
    let log = Component::<Log>::new(2, serde_json::Value::Null);

    let mut flow = Flow::new();
    flow.add_component(Box::new(message))?
        .add_component(Box::new(log))?
        .add_connection(Connection::new(1, 0, 2, 0))?
        .build()?;

    flow.run()?;
    return Ok(());
}
fn main() {
    crate_flow_and_run().unwrap();
}
