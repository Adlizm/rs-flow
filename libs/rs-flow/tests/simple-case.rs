use std::sync::Arc;

use rs_flow::prelude::*;

mod components;
use components::{log::Log, message::Message, CounterLogs};

#[tokio::test]
async fn simple_case() -> Result<()> {
    let a = Component::new(1, Message::new("Hello"));
    let b = Component::new(2, Message::new("World"));
    let log = Component::new(3, Log);

    let conn_a = Connection::by(a.from(0), log.to(0));
    let conn_b = Connection::by(b.from(0), log.to(0));

    let flow = Flow::new()
        .add_component(a)?
        .add_component(b)?
        .add_component(log)?
        .add_connection(conn_a)?
        .add_connection(conn_b)?;

    let flow = Arc::new(flow);

    let mut handlers = vec![];
    for _ in 0..10 {
        let flow = flow.clone();
        let handler = tokio::spawn(async move {
            let global = Global::default().add(CounterLogs { count: 0 });

            let mut global = flow.run(global).await.unwrap();

            let counter = global.remove::<CounterLogs>().unwrap();

            assert!(counter.count == 2);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        let _ = handler.await;
    }

    Ok(())
}
