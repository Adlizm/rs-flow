use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{error::PackageError, 
    serde::{deserialize, serialize, PackageDeserializerError, PackageSerializerError}
};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum Package {
    #[default]
    Empty,
    Number(f64),
    String(String),
    Boolean(bool),
    Bytes(Vec<u8>),
    Array(Vec<Package>),
    Object(HashMap<String, Package>)
}

impl Package {
    /// Try create a [Package] from a type that implement Serialize
    /// 
    /// ```
    /// use rs_flow::package::Package;
    /// use serde::Serialize;
    /// 
    /// #[derive(Serialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u16
    /// }
    /// 
    /// let package = Package::try_from(Person { name: "Boby".to_string(), age: 24 }).unwrap();
    /// let mut person = package.get_object().unwrap(); 
    /// let name = person.remove("name").unwrap().get_string().unwrap();
    /// let age = person.remove("age").unwrap().get_number().unwrap();
    /// 
    /// assert_eq!(&name, "Boby");
    /// assert_eq!(age, 24.0);
    /// ```
    /// 
    pub fn try_from<T: Serialize>(content: T) -> Result<Self, PackageSerializerError> {
        serialize(content)
    }
    /// Try deserialize that [Package] to the type provided
    /// 
    /// ```
    /// use rs_flow::package::Package;
    /// use serde::Deserialize;
    /// 
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u16
    /// }
    /// 
    /// let name = Package::string("Boby");
    /// let age = Package::number(24.0);
    /// let object = Package::object([
    ///     ("name", name),
    ///     ("age", age)
    /// ]);
    /// 
    /// let person: Person = object.try_into().unwrap();
    /// assert_eq!(&person.name, "Boby");
    /// assert_eq!(person.age, 24);
    /// ```
    /// 
    pub fn try_into<T: for<'a> Deserialize<'a>>(self) -> 
        Result<T, PackageDeserializerError> 
    {
        deserialize(self)
    }

    /// Create a empty package
    pub fn empty() -> Self {
        Package::Empty
    }
    /// Create a package with a number
    pub fn number(value: f64) -> Self {
        value.into()
    }
    /// Create a package with a boolean
    pub fn bool(value: bool) -> Self {
        value.into()
    }
    /// Create a package with a string
    pub fn string(value: &str) -> Self {
        value.into()
    }
    /// Create a package with a byte array
    pub fn bytes(value: &[u8]) -> Self {
        value.into()
    }
    /// Create a package with a from a vector of packages
    pub fn array<T: Into<Package>>(value: impl IntoIterator<Item = T>) -> Self {
        Self::Array(value.into_iter().map(Into::into).collect())
    }
    /// Create a package with a object that represent a collection of entries of
    /// that key is a string and tha value is a other package
    pub fn object<T: Into<Package>, K: Into<String>>(value: impl IntoIterator<Item = (K, T)>) -> Self {
        Self::Object(value.into_iter().map(|(k,v)| (k.into(), v.into())).collect())
    }


    /// Return if the package is Empty variant
    pub fn is_empty(&self) -> bool { 
        match self {
            Package::Empty => true,
            _ => false
        }
    }
    /// Return if the package is Number variant
    pub fn is_number(&self) -> bool { 
        match self {
            Package::Number(_) => true,
            _ => false
        }
    }
    /// Return if the package is Boolean variant
    pub fn is_bool(&self) -> bool { 
        match self {
            Package::Boolean(_) => true,
            _ => false
        }
    }
    /// Return if the package is String variant
    pub fn is_string(&self) -> bool { 
        match self {
            Package::String(_) => true,
            _ => false
        }
    }
    /// Return if the package is Bytes variant
    pub fn is_bytes(&self) -> bool { 
        match self {
            Package::Bytes(_) => true,
            _ => false
        }
    }
    /// Return if the package is Array variant
    pub fn is_array(&self) -> bool { 
        match self {
            Package::Array(_) => true,
            _ => false
        }
    }
    /// Return if the package is Object variant
    pub fn is_object(&self) -> bool { 
        match self {
            Package::Object(_) => true,
            _ => false
        }
    }


    /// Return a () if the package is a Empty variant otherwise a error 
    pub fn get_empty(self) -> Result<(), PackageError> { 
        match self {
            Package::Empty => Ok(()),
            _ => Err(PackageError::NotEmpty)
        }
    }
    /// Return a f64 if the package is a Number variant otherwise a error 
    pub fn get_number(self) -> Result<f64, PackageError> { 
        match self {
            Package::Number(number) => Ok(number),
            _ => Err(PackageError::NotNumber)
        }
    }
    /// Return a String if the package is a String variant otherwise a error 
    pub fn get_string(self) -> Result<String, PackageError> { 
        match self {
            Package::String(string) => Ok(string),
            _ => Err(PackageError::NotString)
        }
    }
    /// Return a bool if the package is a Boolean variant otherwise a error 
    pub fn get_bool(self) -> Result<bool, PackageError> { 
        match self {
            Package::Boolean(bool) => Ok(bool),
            _ => Err(PackageError::NotBoolean)
        }
    }
    /// Return a Vec<u8> if the package is a Bytes variant otherwise a error 
    pub fn get_bytes(self) -> Result<Vec<u8>, PackageError> { 
        match self {
            Package::Bytes(bytes) => Ok(bytes),
            _ => Err(PackageError::NotBytes)
        }
    }
    /// Return a Vec<Package> if the package is a Array variant otherwise a error 
    pub fn get_array(self) -> Result<Vec<Package>, PackageError> { 
        match self {
            Package::Array(array) => Ok(array),
            _ => Err(PackageError::NotArray)
        }
    }
    /// Return a HashMap<String, Package>, if the package is a Object variant otherwise a error 
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
impl_from_number!(u8, u16, u32, u64, usize);
impl_from_number!(i8, i16, i32, i64, isize);
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