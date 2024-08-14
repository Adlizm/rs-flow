use std::sync::OnceLock;

use rs_flow::prelude::*;

use super::MyGlobal;

pub struct Log;

#[async_trait]
impl ComponentRunnable for Log {
    type Global = MyGlobal;

    async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next> {
        if let Some(package) = ctx.receive(self.input("message")) {
            println!("{:#}", package.get_string()?);

            ctx.with_mut_global(|global|  { 
                global.count += 1;
            })?;
        }
        Ok(Next::Continue)
    }
}

impl Inputs for Log {
    fn inputs(&self) -> &Ports {
        static INPUTS: OnceLock<Ports> = OnceLock::new();

        INPUTS.get_or_init(|| {
            Ports::new(vec![Port::new(0)])
        })
    }
    fn input(&self, label: &'static str) -> PortId {
        if label == "message" {
            return 0;
        } else {
             panic!("Not found input with label = {label}")
        }
    }
}
impl Outputs for Log {
    fn outputs(&self) -> &Ports {
        static OUTPUTS: OnceLock<Ports> = OnceLock::new();

        OUTPUTS.get_or_init(|| {
            Ports::new(vec![])
        })
    }
    fn output(&self, _label: &'static str) -> PortId {
        panic!("This component not have a output")
    }
}