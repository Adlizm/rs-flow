use std::{fmt::Debug, sync::RwLock};

use crate::error::{Error, Result};

pub trait Global: Send + Sync + 'static {
    type Package: Clone + Debug + Send + Sync;
}

pub(crate) struct GlobalData<G>(RwLock<G>);

impl<G> GlobalData<G> {
    pub(crate) fn from_data(data: G) -> Self {
        Self(RwLock::new(data))
    }

    pub(crate) fn with_global<R>(&self, call: impl FnOnce(&G) -> R) -> Result<R> {
        match self.0.read() {
            Ok(global) => Ok(call(&global)),
            Err(_) => Err(Error::CannotAccessGlobal),
        }
    }

    pub(crate) fn with_mut_global<R>(&self, call: impl FnOnce(&mut G) -> R) -> Result<R> {
        match self.0.write() {
            Ok(mut global) => Ok(call(&mut global)),
            Err(_) => Err(Error::CannotAccessGlobal),
        }
    }

    pub(crate) fn take(self) -> G {
        self.0.into_inner().expect("Global have multiple owners")
    }
}

impl<G> Debug for GlobalData<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GlobalData").finish()
    }
}
