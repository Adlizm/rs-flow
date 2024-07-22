use async_trait::async_trait;

use crate::connection::Point;
use crate::context::ctx::Ctx;
use crate::errors::Result;
use crate::ports::{Inputs, Outputs};

/// Define if next cicle of flow will be executed
///  
/// - If any component return Ok(Next::Break) flow run will be interrupted and return Ok(Global)
/// - If all component return Ok(Next::Continue) flow continue for a more cicle
/// - If any component return Err(_), flow will be interrupted and return that Err
/// 
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Next { 
    #[default]
    Continue, 
    Break
}


///
/// Define when a component is prepared to run.
///
/// - Lazy : Wait for at least one packet received at each input port.
///    
/// - Eager: 
///     - Wait for at least one packet received at each input port.
///     - Wait for all ancestral components to run, is mean that if any 
/// ancestral component is prepared to run, this component will not run.
/// 
/// Obs: If a component does not have an input port, it will be selected 
///      as the flow's entry point, and will be executed once in the first circle.
/// 
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Type {
    #[default]
    Lazy,
    Eager    
}

/// 
/// Id of a component
/// 
pub type Id = usize;


/// 
/// Define the function that will excuted for a component
/// 
/// Global define tha data that this component can be access,
/// the data is like a global state of Flow that any component can be read or write
/// 
/// A Flow hava a unique <Global> type, what means that only component 
/// with the same Self::Global can be use for contruct the flow.
/// 
/// # Examples
/// ```
/// use rs_flow::prelude::*;
/// 
/// struct GlobalA;
/// struct GlobalB;
/// 
/// #[inputs] 
/// #[outputs]
/// struct ComponentA;
/// #[async_trait]
/// impl ComponentRunnable for ComponentA {
///     type Global = GlobalA;
///     async fn run(&self, ctx: Ctx<Self::Global>) -> Result<Next> { 
///         Ok(Next::Continue) 
///     }
/// }
/// 
/// #[inputs] 
/// #[outputs]
/// struct ComponentB;
/// #[async_trait]
/// impl ComponentRunnable for ComponentB {
///     type Global = GlobalB;
///     async fn run(&self, ctx: Ctx<Self::Global>) -> Result<Next> { 
///         Ok(Next::Continue) 
///     }
/// }
/// 
/// let mut flow = Flow::new();
/// flow = flow.add_component(Component::new(1, ComponentA)).unwrap();
/// 
/// // flow = flow.add_component(Component::new(2, ComponentB)).unwrap(); 
/// // Will fail because ComponentA and ComponentB not have same Global
/// 
/// ```
/// 
#[async_trait]
pub trait ComponentRunnable: Send + Sync + Inputs + Outputs + 'static {
    type Global: Send + Sync;

    async fn run(&self, ctx: &mut Ctx<Self::Global>) -> Result<Next>;
}


///
/// Storage the component infos:
/// - Id that indentify a component in a flow,
/// - Type of component 
/// - Traits needed to run (ComponentRunnable + Inputs + Outputs) 
/// 
/// This component cannot be modified once is initialized
/// ```
/// use rs_flow::prelude::*;
/// 
/// struct G;
/// 
/// #[outputs { out1 }]
/// #[inputs { in1, in2 }]
/// struct A;
/// 
/// assert_eq!(A.output("out1"), 0); // first output port
/// assert_eq!(A.input("in1"), 0); // first input port
/// assert_eq!(A.input("in2"), 1); // second input port
/// 
/// #[async_trait]
/// impl ComponentRunnable for A {
///     type Global = G;
///     async fn run(&self, ctx: Ctx<Self::Global>) -> Result<Next> {
///         return Ok(Next::Continue);
///     }
/// }
/// let component1 = Component::new(1, A);   // Type::Lazy
/// let component2 = Component::eager(2, A); // Type::Eeager
/// 
/// assert_eq!(component1.ty(), Type::Lazy);
/// assert_eq!(component2.ty(), Type::Eager);
/// 
/// let c = Connection::by(component1.from("out1"), component2.to("in1"));
/// assert_eq!(Connection::new(1, 0, 2, 0), c);
/// 
/// ```
pub struct Component<G> {
    pub(crate) id: Id,
    pub(crate) data: Box<dyn ComponentRunnable<Global = G>>,
    pub(crate) ty: Type
}

impl<G> Component<G> {
    /// Create a component with Type::Lazy
    pub fn new<T>(id: Id, data: T) -> Self 
        where T: ComponentRunnable<Global = G>
    {
        Self { id, data: Box::new(data), ty: Type::default() }
    }
    /// Create a component with Type::Eager
    pub fn eager<T>(id: Id, data: T) -> Self 
        where T: ComponentRunnable<Global = G>
    {
        Self { id, data: Box::new(data), ty: Type::Eager }
    }

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
    /// Panic if could not found a output port by a label
    pub fn from(&self, label: &'static str) -> Point {
        Point::new(self.id, self.data.output(label))
    }

    /// Return a input point for connection
    /// 
    /// # Panics
    /// Panic if could not found a input port by a label
    pub fn to(&self, label: &'static str) -> Point {
        Point::new(self.id, self.data.input(label))
    }
}
