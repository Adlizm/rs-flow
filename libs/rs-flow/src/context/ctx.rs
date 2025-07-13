use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::context::global::{Global, GlobalData};

use crate::component::{Id, Type};
use crate::error::{Error, Result};
use crate::ports::{Inputs, Outputs, PortId};
use crate::prelude::Component;

pub(crate) enum ReceiveQueue<P> {
    Closed,
    Open(VecDeque<P>),
}
impl<P> ReceiveQueue<P> {
    pub fn new() -> Self {
        Self::Open(VecDeque::new())
    }

    pub fn close(&mut self) {
        *self = Self::Closed
    }

    pub fn push_all(&mut self, packages: &mut VecDeque<P>) {
        match self {
            Self::Open(queue) => queue.append(packages),
            Self::Closed => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Open(queue) => queue.is_empty(),
            Self::Closed => true,
        }
    }

    pub fn get_next(&mut self) -> Option<P> {
        match self {
            Self::Open(queue) => queue.pop_front(),
            Self::Closed => None,
        }
    }

    pub fn get_all(&mut self) -> Vec<P> {
        match self {
            Self::Open(queue) => {
                let mut packages = VecDeque::<P>::new();
                std::mem::swap(queue, &mut packages);

                packages.into()
            }
            Self::Closed => Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Open(queue) => queue.len(),
            Self::Closed => 0,
        }
    }
}

///
/// Provide a interface to send and recieve [Package]'s to/from others [Component]'s
/// and access to read and modify the global data of the [Flow](crate::flow::Flow).
///
pub struct Ctx<G>
where
    G: Global,
{
    pub(crate) id: Id,
    pub(crate) ty: Type,
    pub(crate) send: HashMap<PortId, VecDeque<G::Package>>,
    pub(crate) receive: HashMap<PortId, ReceiveQueue<G::Package>>,
    pub(crate) consumed: bool,
    pub(crate) cicle: u32,

    global: Arc<GlobalData<G>>,
}

impl<G> Ctx<G>
where
    G: Global,
{
    pub(crate) fn from(component: &Component<G>, global: &Arc<GlobalData<G>>) -> Self {
        let send = HashMap::from_iter(
            component
                .outputs
                .iter()
                .map(|port| (port.port, VecDeque::new())),
        );
        let receive = HashMap::from_iter(
            component
                .inputs
                .iter()
                .map(|port| (port.port, ReceiveQueue::new())),
        );
        Self {
            id: component.id,
            ty: component.ty,
            send,
            receive,
            consumed: false,
            cicle: 0,
            global: global.clone(),
        }
    }

    ///
    /// Close this [Port](crate::ports::Port) for receive more package.
    ///
    /// All packages in queue is drop, what means that for the next ctx.receive call
    /// in this port always return None
    ///
    /// # Panics
    ///
    /// Panic if recieve from a [Input](crate::ports::Inputs) Port that not exist in this [Component]
    ///
    pub fn close<I: Inputs>(&mut self, port: I) {
        let port = port.into_port();
        self.close_(port)
    }
    pub fn close_(&mut self, port: PortId) {
        self.consumed = true;

        self.receive
            .get_mut(&port)
            .ok_or(Error::InPortNotFound {
                component: self.id,
                in_port: port,
            })
            .unwrap()
            .close();
    }

    ///
    /// Recieve a [Package] from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if recieve from a [Input](crate::ports::Inputs) Port that not exist in this [Component]
    ///
    pub fn receive<I: Inputs>(&mut self, in_port: I) -> Option<G::Package> {
        let port = in_port.into_port();
        self.receive_(port)
    }
    fn receive_(&mut self, port: PortId) -> Option<G::Package> {
        let package = self
            .receive
            .get_mut(&port)
            .ok_or(Error::InPortNotFound {
                component: self.id,
                in_port: port,
            })
            .unwrap()
            .get_next();

        self.consumed = true;

        package
    }

    ///
    /// Return all [Package]s from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if recieve from a [Input](crate::ports::Inputs) Port that not exist in this [Component]
    ///
    pub fn receive_all<I: Inputs>(&mut self, in_port: I) -> Vec<G::Package> {
        let port = in_port.into_port();
        self.receive_all_(port)
    }
    fn receive_all_(&mut self, port: PortId) -> Vec<G::Package> {
        self.consumed = true;

        self.receive
            .get_mut(&port)
            .ok_or(Error::InPortNotFound {
                component: self.id,
                in_port: port,
            })
            .unwrap()
            .get_all()
    }

    ///
    /// Return the next [Package] in each port [Port](crate::ports::Port) provided
    ///
    /// Return [None] is one of ports not contain a [Package] for receive
    ///
    /// # Panics
    ///
    /// Panic any of [Input](crate::ports::Inputs) Port that exist in this [Component]
    ///
    /// Panic any of [Input](crate::ports::Inputs) Port is repeated
    ///
    pub fn receive_many<I: Inputs, const N: usize>(
        &mut self,
        ports: [I; N],
    ) -> Option<[G::Package; N]> {
        let ports_ids: [PortId; N] = ports.map(|port| port.into_port());
        self.receive_many_(ports_ids)
    }
    fn receive_many_<const N: usize>(&mut self, ports: [PortId; N]) -> Option<[G::Package; N]> {
        let mut ports_ref = [&0; N];
        for i in 0..N {
            ports_ref[i] = &ports[i];
        }

        let queues = self
            .receive
            .get_many_mut(ports_ref)
            .ok_or(Error::InvalidMultipleRecivedPorts {
                component: self.id,
                ports: ports.to_vec(),
            })
            .expect("Already checked that queues exist");

        if queues.iter().any(|queue| queue.is_empty()) {
            return None;
        }

        let mut result = Vec::with_capacity(N);
        for i in 0..N {
            result.push(queues[i].get_next()?);
        }
        Some(result.try_into().expect("Here vec already has N elements"))
    }

    /// Send a [Package] to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package] sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package].
    ///
    /// # Panics
    ///
    /// Panic if send to a [Output](crate::ports::Outputs) Port that not exist in this [Component]
    ///
    pub fn send<O: Outputs, P: Into<G::Package>>(&mut self, out_port: O, package: P) {
        let port = out_port.into_port();
        self.send_(port, package.into());
    }
    fn send_(&mut self, port: PortId, package: G::Package) {
        self.send
            .get_mut(&port)
            .ok_or(Error::OutPortNotFound {
                component: self.id,
                out_port: port,
            })
            .unwrap()
            .push_front(package);
    }

    /// Send all [Package]'s to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package]'s sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package]'s.
    ///
    /// # Panics
    ///
    /// Panic if send to a [Output](crate::ports::Outputs) Port that not exist in this [Component]
    ///
    pub fn send_all<O: Outputs>(&mut self, out_port: O, packages: Vec<G::Package>) {
        let port = out_port.into_port();
        self.send_all_(port, packages);
    }
    fn send_all_(&mut self, port: PortId, packages: Vec<G::Package>) {
        let queue = self
            .send
            .get_mut(&port)
            .ok_or(Error::OutPortNotFound {
                component: self.id,
                out_port: port,
            })
            .unwrap();

        queue.extend(packages.into_iter());
    }

    /// Interface tha provide a way to read the global data of the [Flow](crate::flow::Flow)
    pub fn with_global<R>(&self, call: impl FnOnce(&G) -> R) -> Result<R> {
        self.global.with_global(call)
    }

    /// Interface tha provide a way to read and modify the global data of the [Flow](crate::flow::Flow)
    pub fn with_mut_global<R>(&self, call: impl FnOnce(&mut G) -> R) -> Result<R> {
        self.global.with_mut_global(call)
    }

    #[inline]
    pub fn cicle(&self) -> u32 {
        self.cicle
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }
}
