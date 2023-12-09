use std::sync::Mutex;

use crate::errors::{Errors, Result};

pub trait Global<GB> {
    fn from_data(data: GB) -> Self;

    fn with_global<R>(&self,  call: impl FnOnce(&mut GB) -> R) -> Result<R>;
}

pub struct GlobalAsync<GB>(Mutex<GB>);

impl<GB> Global<GB> for GlobalAsync<GB>  {

    fn from_data(data: GB) -> Self {
        GlobalAsync(Mutex::new(data))
    }

    fn with_global<R>(&self,  call: impl FnOnce(&mut GB) -> R) -> Result<R> {
        match self.0.lock() {
            Ok(mut global) => Ok(call(&mut global)),
            Err(_) => Err(Errors::CannotAccessGlobal.into())
        }
    }

}