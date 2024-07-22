use std::sync::OnceLock;

use rs_flow::prelude::*;

use super::MyGlobal;

pub struct Message {
    pub message: String,
}

#[async_trait]
impl ComponentRunnable for Message {
    type Global = MyGlobal;
    
    async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next> {
        let package = Package::from(self.message.clone());

        ctx.send(self.output("message"), package)?;
        Ok(Next::Continue)
    }
}
impl Inputs for Message {
    fn inputs(&self) -> &Ports {
        static INPUTS: OnceLock<Ports> = OnceLock::new();

        INPUTS.get_or_init(|| {
            Ports::new(vec![])
        })
    }
    fn input(&self, _label: &'static str) -> PortId {
        panic!("This component not have a input")
    }
}
impl Outputs for Message {
    fn outputs(&self) -> &Ports {
        static OUTPUTS: OnceLock<Ports> = OnceLock::new();

        OUTPUTS.get_or_init(|| {
            Ports::new(vec![Port::new(0)])
        })
    }
    fn output(&self, label: &'static str) -> PortId {
        if label == "message" {
            return 0;
        } else {
             panic!("Not found output with label = {label}")
        }
    }
}