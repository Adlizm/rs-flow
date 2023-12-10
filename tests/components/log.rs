use async_trait::async_trait;
use rs_flow::prelude::*;

use super::MyGlobal;

pub struct Log;

#[async_trait]
impl BaseComponent for Log {
    type Global = MyGlobal;

    const INPUTS: &'static [Port] = &[Port::new(0)];
    const OUTPUTS: &'static [Port] = &[];

    async fn run(&self, ctx: &CtxAsync<Self::Global>) -> Result<()> {
        let package = ctx.receive(Self::INPUTS[0])?;

        println!("{:#}", package.content());

        
        let count = ctx.with_global(|global|  { 
            global.count += 1;
            global.count 
        })?;

        println!("Total Messages Recieved: {count}");
        Ok(())
    }
}
