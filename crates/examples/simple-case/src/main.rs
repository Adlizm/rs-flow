use std::sync::Arc;

use rs_flow::prelude::*;

mod components;
use components::{
    MyGlobal,
    log::Log,
    message::Message
};

#[tokio::main]
async fn main() -> Result<()> {

    let message1 = Component::new(1, Message { message: "Hello".to_string() });
    let message2 = Component::new(2, Message { message: "World".to_string() });
    let log = Component::new(3, Log);

    let flow = Flow::new()
        .add_component(message1)?
        .add_component(message2)?
        .add_component(log)?
        .add_connection(Connection::new(1, 0, 3, 0))?
        .add_connection(Connection::new(2, 0, 3, 0))?;

    let flow = Arc::new(flow);
    
    let mut handlers = vec![];
    for _ in 0..10 {
        let tflow = flow.clone();
        let handler = tokio::spawn(async move {
            let global = tflow.run(MyGlobal { count: 0 }).await;
            println!("{:?}", global);
        });
        handlers.push(handler);
    }
    
    for handler in handlers  {
        let _ = handler.await;
    }
    
    Ok(())
}
