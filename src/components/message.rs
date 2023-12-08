use async_trait::async_trait;
use rs_flow::prelude::*;

pub struct Message {
    pub message: String,
}

#[async_trait]
impl BaseComponent for Message {
    const INPUTS: &'static [Port] = &[];
    const OUTPUTS: &'static [Port] = &[Port::from(0, "Message", "Message to Send")];

    async fn run(&self, ctx: &Ctx<AsyncQueues>) -> Result<()> {
        let package = Package::new(self.message.clone());

        ctx.send(Self::OUTPUTS[0], package)?;
        Ok(())
    }
}
