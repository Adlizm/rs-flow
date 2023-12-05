use rs_flow::prelude::*;

use main::components::log::Log;
use main::components::message::Message;

async fn run() -> Result<(), Errors> {
    let message = Component::new(
        1,
        Message {
            message: "Hello World!".to_string(),
        },
    );

    let log = Component::<Log>::new(2, Log {});

    let mut flow = Flow::new();
    flow.add_component(Box::new(message))?
        .add_component(Box::new(log))?
        .add_connection(Connection::new(1, 0, 2, 0))?
        .build()?;

    flow.run().await
}

#[tokio::main]
async fn main() {
    let _ = run().await;
}
