use rs_flow::prelude::*;

use super::CounterLogs;

#[derive(Inputs)]
pub enum In {
    #[description("Message recieved to print in log")]
    Message,
}

pub struct Log;

#[async_trait]
impl ComponentSchema<String> for Log {
    type Inputs = In;
    type Outputs = ();

    async fn run(&self, ctx: &mut Ctx<String>) -> Result<Next> {
        if let Some(package) = ctx.receive(In::Message) {
            println!("{package}");

            ctx.global.with_mut(|counter: &mut CounterLogs| {
                counter.count += 1;
            });
        }
        Ok(Next::Continue)
    }
}
