use rs_flow::prelude::*;

pub mod components;

use components::{
    MyGlobal,
    log::Log,
    message::Message
};

#[tokio::test]
async fn construct() -> Result<()> {
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

mod tests {
    use super::Flow;

    fn is_send<T: Send>(_: T) {}
    fn is_sync<T: Sync>(_: T) {}

    #[derive(Clone)]
    pub struct Global;

    #[test] 
    fn main() {
        let flow = Flow::<Global>::new();

        is_send(Flow::<Global>::run);
        is_sync(Flow::<Global>::run);

        is_send(&flow);
        is_sync(&flow);

        is_send(flow.run(Global{}));
        //is_sync(flow.run(Global{})); // Fails
        
    }
}
