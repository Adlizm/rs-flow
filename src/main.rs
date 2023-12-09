use main::components::MyGlobal;
use rs_flow::prelude::*;

use main::components::log::Log;
use main::components::message::Message;

async fn run() -> Result<()> {
    let message = Component::new(
        1,
        Message {
            message: "Message".to_string(),
        },
    );

    let log = Component::<Log>::new(2, Log {});

    Flow::new()
        .add_component(Box::new(message))?
        .add_component(Box::new(log))?
        .add_connection(Connection::new(1, 0, 2, 0))?
        .run(MyGlobal {count: 0})
        .await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = run().await;
}
