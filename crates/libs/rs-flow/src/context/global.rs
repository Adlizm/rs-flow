use std::sync::RwLock;

use crate::errors::{Errors, Result};

pub(crate) struct Global<GD>(RwLock<GD>);

impl<GD> Global<GD>
    where GD: Send + Sync + 'static
{

    pub(crate) fn from_data(data: GD) -> Self {
        Global(RwLock::new(data))
    }

    pub(crate) fn with_global<R>(&self, call: impl FnOnce(&GD) -> R) -> Result<R> {
        match self.0.read() {
            Ok(global) => Ok(call(&global)),
            Err(_) => Err(Errors::CannotAccessGlobal.into())
        }
    }

    pub(crate) fn with_mut_global<R>(&self,  call: impl FnOnce(&mut GD) -> R) -> Result<R> {
        match self.0.write() {
            Ok(mut global) => Ok(call(&mut global)),
            Err(_) => Err(Errors::CannotAccessGlobal.into())
        }
    }

    pub(crate) fn take(self) -> GD {
        self.0.into_inner().expect("Global have multiple owners")
    }

}