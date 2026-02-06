use std::collections::HashMap;
use std::sync::Arc;

use crate::component::Next;
use crate::connection::{Connection, Connections};
use crate::context::{Ctxs, Global};
use crate::error::{Error, Result, RunResult};
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
/// <img src="https://github.com/Adlizm/rs-flow/raw/main/assets/flow-execution.svg" alt="Flow Execution Logic"/>
///
/// The Flow run in cicles. In each cicle a Set of Component's execute
/// the `run` function defined in trait [Component](crate::component::Component)
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
/// Note that `8` will execute a second time (in 5º cicle) time after recive a Package from `7`
///
///
/// ```
/// use tokio_test;
/// use rs_flow::prelude::*;
///
/// struct Total {
///    value: i32
/// }
///
///
/// #[derive(Inputs, Outputs)]
/// struct DataNumber;
///
///
/// struct Number(i32);
///
/// #[async_trait]
/// impl ComponentSchema<i32> for Number {
///     type Inputs = ();
///     type Outputs = DataNumber;
///
///     async fn run(&self, ctx: &mut Ctx<i32>) -> Result<Next> {
///         ctx.send(DataNumber, self.0);
///         Ok(Next::Continue)
///     }
/// }
///
/// struct Sum;
///
/// #[async_trait]
/// impl ComponentSchema<i32> for Sum {
///     type Inputs = DataNumber;
///     type Outputs = ();
///
///     async fn run(&self, ctx: &mut Ctx<i32>) -> Result<Next> {
///         let mut sum = 0;
///         while let Some(number) = ctx.receive(DataNumber) {
///             sum += number;
///         }
///
///         ctx.global.with_mut(|total: &mut Total| {
///             total.value += sum;
///         });
///
///         Ok(Next::Continue)
///     }
/// }
///
/// tokio_test::block_on(async {
///     let a = Component::new(1, Number(12));
///     let b = Component::new(2, Number(24));
///     let sum = Component::new(3, Sum);
///
///     assert!(rs_flow::ports::Inputs::into_port(&DataNumber) == 0);
///     assert!(rs_flow::ports::Outputs::into_port(&DataNumber) == 0);
///
///     let connection_a = Connection::by(a.from(0), sum.to(0));
///     let connection_b = Connection::by(b.from(0), sum.to(0));
///
///     let global = Global::default().add(Total { value: 0 });
///     let mut global = Flow::new()
///         .add_component(a).unwrap()
///         .add_component(b).unwrap()
///         .add_component(sum).unwrap()
///         .add_connection(connection_a).unwrap()
///         .add_connection(connection_b).unwrap()
///         .run(global).await
///         .unwrap();
///
///     let total = global.remove::<Total>().unwrap();
///     assert!(total.value == 36);
/// });
///
/// ```
///
pub struct Flow<V> {
    components: HashMap<Id, Component<V>>,
    connections: Connections,
}

impl<V> Flow<V> {
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
    ///
    pub fn add_component(mut self, component: Component<V>) -> Result<Self> {
        if self.components.contains_key(&component.id) {
            return Err(Error::ComponentAlreadyExist { id: component.id }.into());
        }
        self.components.insert(component.id, component);
        Ok(self)
    }

    /// Insert a [Connection]
    ///
    /// # Error
    ///
    /// - Error if [Connection] already exist
    /// - Error if the this [Flow] not have a [Component::id] used in [Connection]
    /// - Error if the [Component]'s used in [Connection] not have that Input/Output [Port](crate::ports::Port) defined.
    /// - Error if add a connection create a Loop
    ///
    pub fn add_connection(mut self, connection: Connection) -> Result<Self> {
        if let Some(component) = self.components.get(&connection.from) {
            if !component.outputs.contains(connection.out_port) {
                return Err(Error::OutPortNotFound {
                    component: connection.from,
                    out_port: connection.out_port,
                }
                .into());
            }
        } else {
            return Err(Error::ComponentNotFound {
                id: connection.from,
            }
            .into());
        }

        if let Some(component) = self.components.get(&connection.to) {
            if !component.inputs.contains(connection.in_port) {
                return Err(Error::InPortNotFound {
                    component: connection.from,
                    in_port: connection.in_port,
                }
                .into());
            }
        } else {
            return Err(Error::ComponentNotFound { id: connection.to }.into());
        }

        self.connections.add(connection)?;

        Ok(self)
    }
}

impl<V> Flow<V>
where
    V: Send + Clone + 'static,
{
    ///
    /// Run this Flow
    ///
    /// # Error
    ///
    /// Error if a component return a Error when [run](crate::component::ComponentSchema::run)
    ///
    /// # Panics
    ///
    /// Panic if a component panic when [run](crate::component::ComponentSchema::run)
    ///
    pub async fn run(&self, global: Global) -> RunResult<Global> {
        let global_arc = Arc::new(global);

        let mut contexts = Ctxs::new(&self.components, &self.connections, &global_arc);

        let mut ready_components = contexts.entry_points();
        let mut first = true;

        let mut cicle = 1;
        while !ready_components.is_empty() {
            let mut futures = Vec::with_capacity(ready_components.len());

            for id in ready_components {
                let mut ctx = contexts
                    .borrow(id)
                    .expect("Ready operators never return ids that not exist");

                ctx.consumed = false;
                ctx.cicle = cicle;

                let component = self
                    .components
                    .get(&id)
                    .expect("Ready operators never return ids that not exist");

                futures.push(
                    async move { component.data.run(&mut ctx).await.map(|next| (ctx, next)) },
                );
            }

            let results = futures::future::try_join_all(futures).await?;
            if results.iter().any(|(_, next)| next == &Next::Break) {
                break;
            }

            for (ctx, _) in results {
                if !ctx.consumed && !first {
                    // entry points not have inputs to consume
                    return Err(Box::new(Error::AnyPackageConsumed { component: ctx.id }));
                }
                contexts.give_back(ctx);
            }

            contexts.refresh_queues();

            ready_components = contexts.ready_components(&self.connections);

            first = false;
            cicle += 1;
        }

        drop(contexts);

        let global = Arc::try_unwrap(global_arc)
            .expect("Global no have multiples references, because contexts already drop");
        Ok(global)
    }
}
