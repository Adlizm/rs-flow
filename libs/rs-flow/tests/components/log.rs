use rs_flow::prelude::*;

use super::CounterLogs;

#[derive(Inputs)]
pub enum In {
    #[description("Message recieved to print in log")]
    Message,
}

pub struct Log;

#[async_trait]
impl ComponentSchema for Log {
    type Global = CounterLogs;

    type Inputs = In;
    type Outputs = ();

    async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next> {
        if let Some(package) = ctx.receive(In::Message) {
            println!("{:#}", package.get_string()?);

            ctx.with_mut_global(|global| {
                global.count += 1;
            })?;
        }
        Ok(Next::Continue)
    }
}
