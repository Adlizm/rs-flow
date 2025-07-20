use async_trait::async_trait;

use crate::connection::Point;
use crate::context::Ctx;
use crate::error::RunResult as Result;
use crate::ports::{Inputs, Outputs, PortId, Ports};

/// Define if next cicle of [Flow](crate::flow::Flow) will be executed
///
/// - If any component return <code> Ok([Next::Break]) </code> flow run will be interrupted and return Ok(Global)
/// - If all component return <code> Ok([Next::Continue]) </code> flow continue to run for a more cicle
/// - If any component return <code> Err(_) </code>, flow will be interrupted and return that Error
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Next {
    #[default]
    Continue,
    Break,
}

///
/// Define when a [Component] is prepared to run.
///
/// - [`Lazy`](Type::Lazy) :
///     Wait for at least one [Package](crate::package::Package) received at each input port [Inputs].
///
/// - [`Eager`](Type::Eager):
///     - Wait for at least one [Package](crate::package::Package) received at each input port [Inputs].
///     - Wait for all ancestral components to run, it's means that if any
/// ancestral of this [Component] is prepared to run, this [Component] will not run.
///
/// Obs: If a [Component] does not have an [Inputs] port's, it will be selected
///      as the flow's entry point, and will be executed once in the first cicle.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Type {
    #[default]
    Lazy,
    Eager,
}

///
/// Id of a component
///
pub type Id = usize;

///
/// The [ComponentSchema] trait define the function that will excuted when [`run`](ComponentSchema::run),
/// as like the Inputs and Outputs ports.
///
/// Global define tha data that this component can be access,
/// the data is like a global state of [Flow](crate::flow::Flow) that any [Component] can be read or write
///
/// A [Flow](crate::flow::Flow) hava a unique [Global](ComponentSchema::Global) type, what means that only component
/// with the same Self::Global can be use for contruct the flow.
///
/// # Examples
/// ```
/// use rs_flow::prelude::*;
///
///
/// #[derive(Inputs)]
/// struct In;
///
/// #[derive(Outputs)]
/// enum Out {
///     True,
///     False
/// }
///
/// struct FilterNumbers;
///
/// #[async_trait]
/// impl ComponentSchema for FilterNumbers {
///     type Inputs = In;
///     type Outputs = Out;
///
///     type Global = ();
///
///     async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next> {
///         while let Some(package) = ctx.receive(In) {
///             match package.is_number() {
///                 true => ctx.send(Out::True, package),
///                 false => ctx.send(Out::False, package),
///             }
///         }
///         Ok(Next::Continue)
///     }
/// }
///
/// ```
///
#[async_trait]
pub trait ComponentSchema<V>: Send + Sync + 'static {
    type Inputs: Inputs;
    type Outputs: Outputs;

    async fn run(&self, ctx: &mut Ctx<V>) -> Result<Next>;

    fn description() -> &'static str {
        ""
    }
}

#[async_trait]
pub(crate) trait ComponentRun<V>: Send + Sync + 'static {
    async fn run(&self, ctx: &mut Ctx<V>) -> Result<Next>;
}

#[async_trait]
impl<T: Sized, V> ComponentRun<V> for T
where
    V: Send,
    T: ComponentSchema<V>,
{
    #[inline(always)]
    async fn run(&self, ctx: &mut Ctx<V>) -> Result<Next> {
        self.run(ctx).await
    }
}

///
/// Storage the [Component] infos:
/// - [Id] that identify a operator in a [Flow](crate::flow::Flow),
/// - [Type] of operator
/// - [Inputs] Ports
/// - [Outputs] Ports
/// - [ComponentSchema::run] function
///
///
/// ```
/// use rs_flow::prelude::*;
///
/// #[derive(Inputs)]
/// struct In;
///
/// #[derive(Outputs)]
/// struct Out;
///
/// struct Nothing;
///
/// #[async_trait]
/// impl ComponentSchema<()> for Nothing {
///     type Inputs = In;
///     type Outputs = Out;
///
///     async fn run(&self, ctx: &mut Ctx<()>) -> Result<Next> {
///         return Ok(Next::Continue);
///     }
/// }
/// let a = Component::new(1, Nothing);   // Type::Lazy
/// let b = Component::eager(2, Nothing); // Type::Eeager
///
/// assert_eq!(a.ty(), Type::Lazy);
/// assert_eq!(b.ty(), Type::Eager);
///
/// let connection = Connection::by(a.from(Out.into_port()), b.to(In.into_port()));
/// assert_eq!(connection, Connection::new(1, 0, 2, 0));
///
/// ```
pub struct Component<V> {
    pub(crate) id: Id,
    pub(crate) data: Box<dyn ComponentRun<V>>,
    pub(crate) ty: Type,
    pub(crate) inputs: Ports,
    pub(crate) outputs: Ports,
}

impl<V> Component<V>
where
    V: Send + Clone,
{
    /// Create a component with Type::Lazy
    pub fn new<T>(id: Id, data: T) -> Self
    where
        T: ComponentSchema<V>,
    {
        Self {
            id,
            data: Box::new(data),
            ty: Type::default(),
            inputs: T::Inputs::PORTS,
            outputs: T::Outputs::PORTS,
        }
    }
    /// Create a component with Type::Eager
    pub fn eager<T>(id: Id, data: T) -> Self
    where
        T: ComponentSchema<V>,
    {
        Self {
            id,
            data: Box::new(data),
            ty: Type::Eager,
            inputs: T::Inputs::PORTS,
            outputs: T::Outputs::PORTS,
        }
    }
}

impl<V> Component<V> {
    /// Return id of component
    pub fn id(&self) -> Id {
        self.id
    }

    /// Return type of component
    pub fn ty(&self) -> Type {
        self.ty
    }

    /// Return a output point for connection
    ///
    /// # Panics
    /// Panic if could not found the output port by  label
    pub fn from(&self, port: PortId) -> Point {
        let _ = self.outputs.iter().find(|p| p.port == port).unwrap();
        Point::new(self.id, port)
    }

    /// Return a input point for connection
    ///
    /// # Panics
    /// Panic if could not found the input port
    pub fn to(&self, port: PortId) -> Point {
        let _ = self.inputs.iter().find(|p| p.port == port).unwrap();
        Point::new(self.id, port)
    }
}
