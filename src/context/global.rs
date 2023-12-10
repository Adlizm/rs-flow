use std::sync::Mutex;

use crate::errors::{Errors, Result};

pub trait Global<GD: Send + Sync>: Send + Sync  {
    fn from_data(data: GD) -> Self;

    fn with_global<R>(&self,  call: impl FnOnce(&mut GD) -> R) -> Result<R>;
}

pub struct GlobalAsync<GD>(Mutex<GD>);

impl<GD> Global<GD> for GlobalAsync<GD>  
    where GD: Send + Sync 
{

    fn from_data(data: GD) -> Self {
        GlobalAsync(Mutex::new(data))
    }

    fn with_global<R>(&self,  call: impl FnOnce(&mut GD) -> R) -> Result<R> {
        match self.0.lock() {
            Ok(mut global) => Ok(call(&mut global)),
            Err(_) => Err(Errors::CannotAccessGlobal.into())
        }
    }

}