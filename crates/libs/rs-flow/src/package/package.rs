use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{error::PackageError, serializer::{PackageSerializer, PackageSerializerError}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Package {
    Empty,
    Number(f64),
    String(String),
    Boolean(bool),
    Bytes(Vec<u8>),
    Array(Vec<Package>),
    Object(HashMap<String, Package>)
}

impl Package {
    pub fn empty() -> Self {
        Package::Empty
    }
    
    pub fn from<T: Into<Package>>(value: T) -> Self {
        value.into()
    }

    pub fn create<T: Serialize>(content: T) -> Result<Self, PackageSerializerError> {
        content.serialize(PackageSerializer)
    }

    pub fn array<T: Into<Package>>(value: impl IntoIterator<Item = T>) -> Self {
        Self::Array(value.into_iter().map(Into::into).collect())
    }
    
    pub fn object<T: Into<Package>>(value: impl IntoIterator<Item = (String, T)>) -> Self {
        Self::Object(value.into_iter().map(|(k,v)| (k, v.into())).collect())
    }

    pub fn is_empty(&self) -> bool { 
        match self {
            Package::Empty => true,
            _ => false
        }
    }
    pub fn is_number(&self) -> bool { 
        match self {
            Package::Number(_) => true,
            _ => false
        }
    }
    pub fn is_bool(&self) -> bool { 
        match self {
            Package::Boolean(_) => true,
            _ => false
        }
    }
    pub fn is_string(&self) -> bool { 
        match self {
            Package::String(_) => true,
            _ => false
        }
    }
    pub fn is_array(&self) -> bool { 
        match self {
            Package::Array(_) => true,
            _ => false
        }
    }
    pub fn is_object(&self) -> bool { 
        match self {
            Package::Object(_) => true,
            _ => false
        }
    }


    pub fn get_number(self) -> Result<f64, PackageError> { 
        match self {
            Package::Number(number) => Ok(number),
            _ => Err(PackageError::NotNumber)
        }
    }
    pub fn get_string(self) -> Result<String, PackageError> { 
        match self {
            Package::String(string) => Ok(string),
            _ => Err(PackageError::NotString)
        }
    }
    pub fn get_bool(self) -> Result<bool, PackageError> { 
        match self {
            Package::Boolean(bool) => Ok(bool),
            _ => Err(PackageError::NotBoolean)
        }
    }
    pub fn get_bytes(self) -> Result<Vec<u8>, PackageError> { 
        match self {
            Package::Bytes(bytes) => Ok(bytes),
            _ => Err(PackageError::NotBytes)
        }
    }
    pub fn get_array(self) -> Result<Vec<Package>, PackageError> { 
        match self {
            Package::Array(array) => Ok(array),
            _ => Err(PackageError::NotArray)
        }
    }
    pub fn get_object(self) -> Result<HashMap<String, Package>, PackageError> { 
        match self {
            Package::Object(object) => Ok(object),
            _ => Err(PackageError::NotObject)
        }
    }

}

/// Packages number implmentations
macro_rules! impl_from_number {
    ($($ty: ty),+) => {
        $(
            impl From<$ty> for Package {
                fn from(value: $ty) -> Self {
                    Package::Number(value as f64)
                }
            }
        )+
    };
}
impl_from_number!(u8, u16, u32, u64, u128, usize);
impl_from_number!(i8, i16, i32, i64, i128, isize);
impl_from_number!(f32, f64);

/// Packages boolean implmentations
impl From<bool> for Package {
    fn from(value: bool) -> Self {
        Package::Boolean(value)
    }
}


/// Packages string implementations 
impl From<String> for Package {
    fn from(value: String) -> Self {
        Package::String(value)
    }
}
impl From<&str> for Package {
    fn from(value: &str) -> Self {
        Package::String(value.to_owned())
    }
}

/// Packages bytes implementations 
impl From<Vec<u8>> for Package {
    fn from(value: Vec<u8>) -> Self {
        Package::Bytes(value)
    }
}
impl From<&[u8]> for Package {
    fn from(value: &[u8]) -> Self {
        Package::Bytes(value.into())
    }
}
impl<const C: usize> From<[u8; C]> for Package {
    fn from(value: [u8; C]) -> Self {
        Package::Bytes(value.into())
    }
}