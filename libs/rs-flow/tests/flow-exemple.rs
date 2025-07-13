use rs_flow::prelude::*;

struct Nothing;
impl Global for Nothing {
    type Package = ();
}

#[derive(Inputs)]
struct In; // A unique port of input

#[derive(Outputs)]
struct Out; // A unique port of output

struct Red;
struct Green;
struct Blue;

#[async_trait]
impl<G> ComponentSchema<G> for Red
where
    G: Global<Package = ()>,
{
    type Inputs = ();
    type Outputs = Out;

    async fn run(&self, ctx: &mut Ctx<G>) -> Result<Next> {
        println!(
            "Runinng component Red({}) in {} cicle",
            ctx.id(),
            ctx.cicle()
        );

        ctx.send(Out, ());
        Ok(Next::Continue)
    }
}

#[async_trait]
impl<G> ComponentSchema<G> for Green
where
    G: Global<Package = ()>,
{
    type Inputs = In;
    type Outputs = ();

    async fn run(&self, ctx: &mut Ctx<G>) -> Result<Next> {
        println!(
            "Runinng component Green({}) in {} cicle",
            ctx.id(),
            ctx.cicle()
        );

        let _ = ctx.receive_all(In);

        Ok(Next::Continue)
    }
}

#[async_trait]
impl<G> ComponentSchema<G> for Blue
where
    G: Global<Package = ()>,
{
    type Inputs = In;
    type Outputs = Out;

    async fn run(&self, ctx: &mut Ctx<G>) -> Result<Next> {
        println!(
            "Runinng component Blue({}) in {} cicle",
            ctx.id(),
            ctx.cicle()
        );

        let _ = ctx.receive_all(In);
        ctx.send(Out, ());

        Ok(Next::Continue)
    }
}

#[tokio::test]
async fn flow_example() -> Result<()> {
    let one = Component::new(1, Red);
    let two = Component::new(2, Red);

    let three = Component::new(3, Blue);
    let four = Component::new(4, Blue);
    let five = Component::eager(5, Blue);
    let six = Component::new(6, Blue);
    let seven = Component::new(7, Blue);
    let eight = Component::new(8, Blue);

    let nine = Component::new(9, Green);
    let ten = Component::new(10, Green);

    let connections = [
        Connection::by(one.from(0), three.to(0)),
        Connection::by(one.from(0), five.to(0)),
        Connection::by(two.from(0), four.to(0)),
        Connection::by(four.from(0), five.to(0)),
        Connection::by(three.from(0), eight.to(0)),
        Connection::by(five.from(0), six.to(0)),
        Connection::by(five.from(0), seven.to(0)),
        Connection::by(seven.from(0), eight.to(0)),
        Connection::by(seven.from(0), nine.to(0)),
        Connection::by(eight.from(0), ten.to(0)),
    ];

    let components = [one, two, three, four, five, six, seven, eight, nine, ten];

    let mut flow = Flow::new();
    for component in components {
        flow = flow.add_component(component)?;
    }
    for connection in connections {
        flow = flow.add_connection(connection)?;
    }

    println!("Initing Flow::run");

    let _ = flow.run(Nothing).await?;

    println!("Flow::run finished");

    Ok(())
}
