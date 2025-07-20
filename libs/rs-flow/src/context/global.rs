use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::RwLock,
};

#[derive(Debug, Default)]
pub struct Global {
    vars: HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>,
}

impl Global {
    pub fn add<T: Any + Send + Sync>(mut self, value: T) -> Self {
        self.vars
            .entry(value.type_id())
            .insert_entry(RwLock::new(Box::new(value)));

        self
    }

    pub fn remove<T: Any + Send + Sync>(&mut self) -> Option<T> {
        let any = self.vars.remove(&TypeId::of::<T>())?;

        // We have &mut self, then anyone have the &self to lock this value
        // (only us), since anyone hold the lock, then we can destroiy the RwLock
        // to get the inner value T;
        //
        // Is ok to panic with the lock is poisoned, cause with is poisoned one
        // of components have panic using this value.
        let value = any.into_inner().unwrap().downcast::<T>().unwrap();

        Some(*value)
    }

    pub fn with<T, F, R>(&self, f: F) -> Option<R>
    where
        T: Any + Send + Sync,
        F: FnOnce(&T) -> R,
    {
        let guard = self.vars.get(&TypeId::of::<T>())?.read().unwrap();
        let boxv = guard.as_ref();
        let var = boxv.downcast_ref::<T>().unwrap();

        Some(f(var))
    }

    pub fn with_mut<T, F, R>(&self, f: F) -> Option<R>
    where
        T: Any + Send + Sync,
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.vars.get(&TypeId::of::<T>())?.write().unwrap();
        let boxv = guard.as_mut();
        let var = boxv.downcast_mut::<T>().unwrap();

        Some(f(var))
    }
}
