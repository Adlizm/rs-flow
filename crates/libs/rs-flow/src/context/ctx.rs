use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::context::global::Global;

use crate::component::{Id, Type};
use crate::errors::{Errors, Result};
use crate::package::Package;
use crate::ports::PortId;
use crate::prelude::Component;


pub struct Ctx<GD: Send + Sync> {
    pub(crate) id: Id,
    pub(crate) ty: Type,
    pub(crate) send: HashMap<PortId, VecDeque<Package>>,
    pub(crate) receive: HashMap<PortId, VecDeque<Package>>,

    global: Arc<Global<GD>>,
}

impl<GD> Ctx<GD> 
    where GD: Send + Sync + 'static
{
    pub(crate) fn from(component: &Component<GD>, global: &Arc<Global<GD>>) -> Self {
        let send = HashMap::from_iter(
            component.data.outputs().0.iter().map(|port| (port.port, VecDeque::new()))
        );
        let receive = HashMap::from_iter(
            component.data.inputs().0.iter().map(|port| (port.port, VecDeque::new()))
        );
        Self {
            id: component.id,
            ty: component.ty,
            send,
            receive,
            global: global.clone(),
        }
    }
    
    pub fn receive(&mut self, in_port: PortId) -> Result<Option<Package>> {
        let package = self.receive.get_mut(&in_port)
            .ok_or(Errors::QueueNotCreated { 
                component: self.id, port: in_port 
            })?
            .pop_front();
        Ok(package)
    }
    
    pub fn send(&mut self, out_port: PortId, package: Package) -> Result<()> {
        self.send.get_mut(&out_port)
            .ok_or(Errors::QueueNotCreated { 
                component: self.id, port: out_port 
            })?
            .push_front(package);
        Ok(())
    }
    

    pub fn with_global<R>(&self, call: impl FnOnce(&GD) -> R) -> Result<R> {
        self.global.with_global(call)
    }
    
    pub fn with_mut_global<R>(&self,  call: impl FnOnce(&mut GD) -> R) -> Result<R> {
        self.global.with_mut_global(call)
    }

}
