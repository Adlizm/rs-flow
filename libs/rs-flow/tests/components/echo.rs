use rs_flow::prelude::*;

/// Simple echo component for tests.
///
/// - Receives packages on `In::Data` and forwards them unchanged to `Out::Data`.
#[derive(Inputs)]
pub enum In {
    #[description("Data received to be echoed")]
    Data,
}

#[derive(Outputs)]
pub enum Out {
    #[description("Echoed data")]
    Data,
}

pub struct Echo;

#[async_trait]
impl ComponentSchema<String> for Echo {
    type Inputs = In;
    type Outputs = Out;

    async fn run(&self, ctx: &mut Ctx<String>) -> Result<Next> {
        while let Some(pkg) = ctx.receive(In::Data) {
            ctx.send(Out::Data, pkg);
        }
        Ok(Next::Continue)
    }
}
