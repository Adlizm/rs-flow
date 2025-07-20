use rs_flow::prelude::*;

#[derive(Outputs)]
pub enum Out {
    #[description("Message send to print in log")]
    Message,
}

pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[async_trait]
impl ComponentSchema<String> for Message {
    type Inputs = ();
    type Outputs = Out;

    async fn run(&self, ctx: &mut Ctx<String>) -> Result<Next> {
        ctx.send(Out::Message, self.message.clone());
        Ok(Next::Continue)
    }
}
