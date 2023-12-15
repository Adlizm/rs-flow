use async_trait::async_trait;
use rs_flow::prelude::*;

use super::MyGlobal;

pub struct Message {
    pub message: String,
}

#[async_trait]
impl BaseComponent for Message {
    type Global = MyGlobal;
    
    const DESCRIPTION: &'static str = "Send a package with message";
    const INPUTS: &'static [Port] = &[];
    const OUTPUTS: &'static [Port] = &[Port::from(0, "Message", "Message to Send")];

    async fn run(&self, ctx: &CtxAsync<Self::Global>) -> Result<()> {
        let package = Package::new(self.message.clone());

        ctx.send(&Self::OUTPUTS[0], package)?;
        Ok(())
    }
}
