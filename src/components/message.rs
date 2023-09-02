use rs_flow::prelude::*;
use serde_json::Value;

pub struct Message;

impl BaseComponent for Message {
    const INPUTS: &'static [InPort] = &[];
    const OUTPUTS: &'static [OutPort] = &[OutPort { port: 0 }];

    fn run(data: &Value, ctx: &Ctx) -> Result<(), Errors> {
        let package = Package::new(data);

        ctx.send(Self::OUTPUTS[0], package)?;
        Ok(())
    }
}
