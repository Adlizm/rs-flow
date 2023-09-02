use rs_flow::prelude::*;
use serde_json::Value;

pub struct Log;

impl BaseComponent for Log {
    const INPUTS: &'static [InPort] = &[InPort { port: 0 }];
    const OUTPUTS: &'static [OutPort] = &[];

    fn run(_data: &Value, ctx: &Ctx) -> Result<(), Errors> {
        let package = ctx.receive(Self::INPUTS[0])?;

        println!("{:#}", package.content());
        Ok(())
    }
}
