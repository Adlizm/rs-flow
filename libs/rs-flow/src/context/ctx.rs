use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::context::global::Global;

use crate::component::{Id, Type};
use crate::error::{Error, Result};
use crate::package::Package;
use crate::ports::{Inputs, Outputs, PortId};
use crate::prelude::Component;

///
/// Provide a interface to send and recieve [Package]'s to/from others [Component]'s
/// and access to read and modify the global data of the [Flow](crate::flow::Flow).
///
pub struct Ctx<G> {
    pub(crate) id: Id,
    pub(crate) ty: Type,
    pub(crate) send: HashMap<PortId, VecDeque<Package>>,
    pub(crate) receive: HashMap<PortId, VecDeque<Package>>,
    pub(crate) consumed: bool,
    pub(crate) cicle: u32,

    global: Arc<Global<G>>,
}

impl<G> Ctx<G> {
    pub(crate) fn from(component: &Component<G>, global: &Arc<Global<G>>) -> Self {
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
                .map(|port| (port.port, VecDeque::new())),
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
    /// Recieve a [Package] from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if recieve from a [Input](crate::ports::Inputs) Port that not exist in this [Component]
    ///
    pub fn receive<I: Inputs>(&mut self, in_port: I) -> Option<Package> {
        let port = in_port.into_port();
        self.receive_in_port(port)
    }
    fn receive_in_port(&mut self, port: PortId) -> Option<Package> {
        let package = self
            .receive
            .get_mut(&port)
            .ok_or(Error::QueueNotCreated {
                component: self.id,
                port,
            })
            .unwrap()
            .pop_front();

        self.consumed = true;

        package
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
    pub fn send<O: Outputs>(&mut self, out_port: O, package: Package) {
        let port = out_port.into_port();
        self.send_in_port(port, package);
    }
    fn send_in_port(&mut self, port: PortId, package: Package) {
        self.send
            .get_mut(&port)
            .ok_or(Error::QueueNotCreated {
                component: self.id,
                port,
            })
            .unwrap()
            .push_front(package);
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
