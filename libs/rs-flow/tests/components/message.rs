use rs_flow::prelude::*;

use super::CounterLogs;

#[derive(Outputs)]
pub enum Out {
    #[description("Message send to print in log")]
    Message,
}

pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new<'a>(message: &'a str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[async_trait]
impl ComponentSchema for Message {
    type Global = CounterLogs;

    type Inputs = ();
    type Outputs = Out;

    async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next> {
        ctx.send(Out::Message, self.message.clone().into());
        Ok(Next::Continue)
    }
}
