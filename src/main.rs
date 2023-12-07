use rs_flow::prelude::*;

use main::components::log::Log;
use main::components::message::Message;

async fn run() -> Result<(), Errors> {
    let message_1 = Component::new(
        1,
        Message {
            message: "Message 1".to_string(),
        },
    );
    let message_2 = Component::new(
        2,
        Message {
            message: "Message 2".to_string(),
        },
    );

    let log = Component::<Log>::new(3, Log {});

    Flow::new()
        .add_component(Box::new(message_1))?
        .add_component(Box::new(message_2))?
        .add_component(Box::new(log))?
        .add_connection(Connection::new(1, 0, 3, 0))?
        .add_connection(Connection::new(2, 0, 3, 0))?
        .run()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = run().await;
}
