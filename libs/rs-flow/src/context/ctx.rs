use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Arc;

use crate::context::global::Global;
use crate::context::queue::ReceiveQueue;

use crate::component::{Id, Type};
use crate::error::Error;
use crate::ports::{Inputs, Outputs, PortId};
use crate::prelude::{Component, ComponentSchema};

pub struct Ctx<C>
where
    C: ComponentSchema,
{
    inner: InnerCtx<C::Package>,
}

///
/// Provide a interface to send and recieve [Package]'s to/from others [Component]'s
/// and access to read and modify the global data of the [Flow](crate::flow::Flow).
///
pub(crate) struct InnerCtx<V> {
    pub(crate) id: Id,
    pub(crate) ty: Type,
    pub(crate) send: HashMap<PortId, VecDeque<V>>,
    pub(crate) receive: HashMap<PortId, ReceiveQueue<V>>,
    pub(crate) consumed: bool,
    pub(crate) cicle: u32,

    pub(crate) global: Arc<Global>,
}

impl<V> InnerCtx<V> {
    pub(crate) fn from(component: &Component<V>, global: Arc<Global>) -> Self {
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
            global,
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
    pub(crate) fn close(&mut self, port: PortId) {
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
    /// Recieve a [Package](ComponentSchema::Package) from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if recieve from a Port that not exist in this [Component]
    ///
    pub(crate) fn receive(&mut self, port: PortId) -> Option<V> {
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
    /// Panic if recieve from a Port that not exist in this [Component]
    ///
    pub(crate) fn receive_all(&mut self, port: PortId) -> Vec<V> {
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
    /// Return the next [Package](ComponentSchema::Package) in each port [Port](crate::ports::Port) provided
    ///
    /// Return [None] is one of ports not contain a [Package] for receive
    ///
    /// # Panics
    ///
    /// Panic any of Port not exist in this [Component]
    ///
    /// Panic any of Port is repeated
    ///
    pub(crate) fn receive_many<const N: usize>(&mut self, ports: [PortId; N]) -> Option<[V; N]> {
        let mut ports_ref = [&0; N];
        for i in 0..N {
            ports_ref[i] = &ports[i];
        }

        let queues = self.receive.get_disjoint_mut(ports_ref);

        if queues
            .iter()
            .any(|queue| queue.as_ref().is_none_or(|q| q.is_empty()))
        {
            return None;
        }

        let mut result = Vec::with_capacity(N);
        for queue in queues.into_iter() {
            if let Some(queue) = queue {
                let item = queue.get_next().expect("Queue is not empty by previus if");
                result.push(item);
            } else {
                unreachable!("Already checked in previus if");
            };
        }

        Some(
            result
                .try_into()
                .map_err(|_| ())
                .expect("Here vec already has N elements"),
        )
    }

    /// Send a [Package](ComponentSchema::Package) to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package] sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package].
    ///
    /// # Panics
    ///
    /// Panic if send to a Port that not exist in this [Component]
    ///
    pub(crate) fn send(&mut self, port: PortId, package: V) {
        self.send
            .get_mut(&port)
            .ok_or(Error::OutPortNotFound {
                component: self.id,
                out_port: port,
            })
            .unwrap()
            .push_front(package);
    }

    /// Send all [Package](ComponentSchema::Package)'s to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package]'s sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package]'s.
    ///
    /// # Panics
    ///
    /// Panic if send to a [Output](crate::ports::Outputs) Port that not exist in this [Component]
    ///
    pub(crate) fn send_all(&mut self, port: PortId, packages: Vec<V>) {
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
}

impl<C: ComponentSchema> Ctx<C> {
    ///
    /// Close this [Port](crate::ports::Port) for receive more package.
    ///
    /// All packages in queue is drop, what means that for the next ctx.receive call
    /// in this port always return None
    ///
    /// # Panics
    ///
    /// Panic if this [Component] not has a Input port.
    ///
    pub fn close(&mut self, port: C::Inputs) {
        self.inner.close(port.into_port());
    }

    ///
    /// Recieve a [Package](ComponentSchema::Package) from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if this [Component] not has a Input port.
    ///
    #[inline]
    pub fn receive(&mut self, port: C::Inputs) -> Option<C::Package> {
        self.inner.receive(port.into_port())
    }

    ///
    /// Return all [Package]s from a [Port](crate::ports::Port)
    ///
    /// # Panics
    ///
    /// Panic if this [Component] not has a Input port.
    ///
    #[inline]
    pub fn receive_all(&mut self, port: C::Inputs) -> Vec<C::Package> {
        self.inner.receive_all(port.into_port())
    }

    ///
    /// Return the next [Package](ComponentSchema::Package) in each port [Port](crate::ports::Port) provided
    ///
    /// Return [None] is one of ports not contain a [Package] for receive
    ///
    /// # Panics
    ///
    /// Panic if [Component] not has a Input port.
    ///
    /// Panic any of Port is repeated
    ///
    #[inline]
    pub fn receive_many<const N: usize>(
        &mut self,
        ports: [C::Inputs; N],
    ) -> Option<[C::Package; N]> {
        self.inner.receive_many(ports.map(|p| p.into_port()))
    }

    /// Send a [Package](ComponentSchema::Package) to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package] sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package].
    ///
    /// # Panics
    ///
    /// Panic if this [Component](ComponentSchema) not has a Output port.
    ///
    pub fn send(&mut self, port: C::Outputs, package: C::Package) {
        self.inner.send(port.into_port(), package);
    }

    /// Send all [Package](ComponentSchema::Package)'s to a [Port](crate::ports::Port), if one [Component] is connected to this port than he
    /// can recieve that [Package]'s sent.
    ///
    /// If more than one components is connected in this port, each one recieve a copy of this [Package]'s.
    ///
    /// # Panics
    ///
    /// Panic if this [Component](ComponentSchema) not has a Output port.
    ///
    #[inline]
    pub fn send_all(&mut self, port: C::Outputs, packages: Vec<C::Package>) {
        self.inner.send_all(port.into_port(), packages);
    }

    pub fn global(&self) -> &Global {
        self.inner.global.deref()
    }

    #[inline]
    pub fn cicle(&self) -> u32 {
        self.inner.cicle
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.inner.id
    }
}
