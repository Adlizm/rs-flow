use rs_flow::prelude::*;

pub struct Message {
    pub message: String,
}

impl BaseComponent for Message {
    const INPUTS: &'static [InPort] = &[];
    const OUTPUTS: &'static [OutPort] = &[OutPort { port: 0 }];

    fn run(&self, ctx: &Ctx) -> Result<(), Errors> {
        let package = Package::new(self.message.clone());

        ctx.send(Self::OUTPUTS[0], package)?;
        Ok(())
    }
}
