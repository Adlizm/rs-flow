use std::collections::HashMap;
use std::sync::Arc;

use crate::component::Next;
use crate::connection::{Connection, Connections};
use crate::context::global::Global;
use crate::context::Ctxs;
use crate::error::{FlowError, Result, RunResult};
use crate::prelude::{Component, Id};


///
/// A Flow provided a interface to run [Component]'s in a defined order.
/// 
/// That order is defined by the [Inputs](crate::ports::Inputs) and 
/// [Outputs](crate::ports::Outputs) port's of the component's and the 
/// [Connection]'s between the [Component]'s.
/// 
/// The image bellow show the logic of [Flow] execution and when each [Component] will run.
/// 
/// <img src="https://github.com/Adlizm/rs-flow/assets/flow-execution.svg" alt="Flow Execution Logic"/>
/// 
/// The Flow run in cicles. In each cicle a Set of Component's execute
/// the `run` function defined in trait [ComponentRunnable](crate::component::ComponentRunnable)
/// 
/// The image shows a Flow that have 10 componets with 3 differents types: Red, Green and Blue, and:
///  - Red components have only a Output port, wihtout Inputs
///  - Green components have only a Input port, wihtout Outputs
///  - Blue components have one Input and one Output port
/// 
/// For the next explication, we be consider that:
///  - When Red `run` a [Package](crate::package::Package) is sent to your Output port.
///  - When Green `run` consume all [Package](crate::package::Package)'s sended to your Input port.
///  - When Blue `run` consume all [Package](crate::package::Package)'s sended to your Input port and send a Package to your Output port.
/// 
/// In the First cicle the Component's `1` and `2` will the run. In fact every [Component]
/// without a [Input](crate::ports::Inputs) ports will executed once in the first cicle. 
/// Note that `1` have two [Connection]'s (to `3` and `5`) and each one recieve a copy of the [Package](crate::package::Package) sent.  
/// 
/// In the Second cicle the Component's `3` and `4` will run, because both recieve a 
/// [Package](crate::package::Package) in your [Input](crate::ports::Inputs) port.
/// Note that `5` also recieve a Package but he is defined like [Type::Eager](crate::component::Type::Eager),
/// for that he is waiting for `4` to execute.
/// 
/// In the Third cicle th Component's `5` and `8` run, because both have Packages in your
/// Input port (`5` sended by `1` and `4`, `8` sended by `3`), in this case `5` will run
/// because `1`,`2` and `4` already run. 
/// 
/// This logic will be repeated until there are no more components that can be executed.
/// (read [Type](crate::component::Type) and [Next]). 
/// 
/// Note that `8` will execute a second (in 5º cicle) time after recive a Package from `7`
/// 
/// 
/// ```
/// use tokio_test;
/// use rs_flow::prelude::*;
///
/// struct Total { 
///    value: f64
/// }
///
/// #[inputs]
/// #[outputs { data }]
/// struct One;
///
/// #[async_trait]
/// impl ComponentRunnable for One {
///     type Global = Total;
///     async fn run(&self, ctx: &mut Ctx<Total>) -> Result<Next> {
///         ctx.send(self.output("data"), 1.into());
///         Ok(Next::Continue)
///     }
/// }
/// 
/// #[inputs { a , b }]
/// #[outputs]
/// struct Sum;
/// 
/// #[async_trait]
/// impl ComponentRunnable for Sum {
///     type Global = Total;
///     async fn run(&self, ctx: &mut Ctx<Total>) -> Result<Next> {
///         let a = ctx.receive(self.input("a")).unwrap().get_number()?;
///         let b = ctx.receive(self.input("b")).unwrap().get_number()?;
///
///         ctx.with_mut_global(|total| {
///             total.value += a + b;
///         })?;
/// 
///         Ok(Next::Continue)
///     }
/// }
///
/// tokio_test::block_on(async {
///     let a = Component::new(1, One);
///     let b = Component::new(2, One);
///     let sum = Component::new(3, Sum);
///
///     let connection_a = Connection::by(a.from("data"), sum.to("a"));
///     let connection_b = Connection::by(b.from("data"), sum.to("b"));
///
///     let total = Flow::new()
///         .add_component(a).unwrap()
///         .add_component(b).unwrap()
///         .add_component(sum).unwrap()
///         .add_connection(connection_a).unwrap()
///         .add_connection(connection_b).unwrap()
///         .run(Total { value: 0.0 }).await
///         .unwrap();
///
///     assert!(total.value == 2.0);
/// });
/// 
/// ```
/// 
pub struct Flow<G> 
    where G: Sync + Send
{
    components: HashMap<Id, Component<G>>,
    connections: Connections,
}


impl<G> Flow<G> 
    where G: Sync + Send + 'static
{
    /// Create a flow without components or connections
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            connections: Connections::new(),
        }
    }

    /// Insert a [Component]
    /// 
    /// # Error
    /// 
    /// Error if the [Component::id] is already used
    pub fn add_component(mut self, component: Component<G>) -> Result<Self> {
        if self.components.contains_key(&component.id) {
            return Err(FlowError::ComponentAlreadyExist { id: component.id }.into());
        }
        self.components.insert(component.id ,component);
        Ok(self)
    }

    /// Insert a [Connection]
    /// 
    /// # Error
    /// 
    /// - Error if [Connection] already exist
    /// - Error if the this [Flow] not have a [Component::id] used in [Connection]
    /// - Error if the [Component]'s used in [Connection] not have the Input/Output [Port](crate::ports::Port) defined.
    /// - Error if add a connection create a Loop 
    pub fn add_connection(mut self, connection: Connection) -> Result<Self> {
        if let Some(component) = self.components.get(&connection.from) {
            if !component.data.outputs().contains(connection.out_port)
            {
                return Err(FlowError::OutPortNotFound {
                    component: connection.from,
                    out_port: connection.out_port,
                }
                .into());
            }
        } else {
            return Err(FlowError::ComponentNotFound {
                id: connection.from,
            }
            .into());
        }

        if let Some(component) = self.components.get(&connection.to){
            if !component.data.inputs().contains(connection.in_port)
            {
                return Err(FlowError::InPortNotFound {
                    component: connection.from,
                    in_port: connection.in_port,
                }
                .into());
            }
        } else {
            return Err(FlowError::ComponentNotFound { id: connection.to }.into());
        }

        self.connections.add(connection)?;

        Ok(self)
    }

    /// 
    /// Run this Flow 
    /// 
    /// # Error
    /// 
    /// Error if a component return a Error when [run](crate::component::ComponentRunnable::run)
    /// 
    /// # Panics
    /// 
    /// Panic if a component panic when [run](crate::component::ComponentRunnable::run)
    /// 
    pub async fn run(&self, global: G) -> RunResult<G> {
        let global_arc = Arc::new(Global::from_data(global));
        
        let mut contexts = Ctxs::new(&self.components, &self.connections, &global_arc);

        let mut ready_components = contexts.entry_points();
        let mut first = true;

        while !ready_components.is_empty() {
            let mut futures = Vec::with_capacity(ready_components.len());

            for id in ready_components {
                let mut ctx = contexts.borrow(id)
                    .expect("Ready component never return ids that not exist");

                ctx.consumed = false;

                let component = self.components.get(&id)
                    .expect("Ready component never return ids that not exist");

                futures.push(async move {
                    component.data.run(&mut ctx).await
                        .map(|next| (ctx, next))
                });
            }

            let results = futures::future::try_join_all(futures).await?;
            if results.iter().any(|(_, next)| next == &Next::Break) {
                break;
            }

            for (ctx, _) in results {
                if !ctx.consumed && !first { // entry points not have inputs to consume
                    return Err(Box::new(FlowError::AnyPackageConsumed { component: ctx.id }));
                }
                contexts.give_back(ctx);
            }

            contexts.refresh_queues();

            ready_components = contexts.ready_components(&self.connections);

            first = false;
        }
        
        drop(contexts);
        
        let global = Arc::try_unwrap(global_arc)
            .expect("Global have multiples owners, but contexts already drop")
            .take();
        Ok(global)
    }
}