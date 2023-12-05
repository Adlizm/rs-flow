use async_trait::async_trait;
use rs_flow::prelude::*;

pub struct Log;

#[async_trait]
impl BaseComponent for Log {
    const INPUTS: &'static [InPort] = &[InPort { port: 0 }];
    const OUTPUTS: &'static [OutPort] = &[];

    async fn run(&self, ctx: &Ctx) -> Result<(), Errors> {
        let package = ctx.receive(Self::INPUTS[0])?;

        println!("{:#}", package.content());
        Ok(())
    }
}
