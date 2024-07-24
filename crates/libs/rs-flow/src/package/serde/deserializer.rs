use std::{collections::{hash_map::IntoIter, HashMap}, fmt::Display};

use serde::{
    de::{
        DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess
    }, 
    Deserialize, 
    Deserializer
};
use thiserror::Error;

use crate::package::{error::PackageError, Package};

#[derive(Debug, Error)]
#[error("Package could not be deserialized, cause: {cause:?}")]
pub struct PackageDeserializerError {
    cause: String
}

impl From<PackageError> for PackageDeserializerError {
    fn from(value: PackageError) -> Self {
        Self { cause: value.to_string() }
    }
}

impl serde::de::Error for PackageDeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self { cause: msg.to_string() }
    }
}


pub fn deserialize<T: for<'a> Deserialize<'a>>(package: Package) -> 
    Result<T, PackageDeserializerError> 
{
    T::deserialize(package)
}


// region: impl Deserializer

impl<'de> Deserializer<'de> for Package {
    type Error = PackageDeserializerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> 
    {
        match self {
            Package::Empty => self.deserialize_unit(visitor),
            Package::Number(_) => self.deserialize_f64(visitor),
            Package::String(_) => self.deserialize_string(visitor),
            Package::Boolean(_) => self.deserialize_bool(visitor),
            Package::Bytes(_) => self.deserialize_bytes(visitor),
            Package::Array(_) => self.deserialize_seq(visitor),
            Package::Object(_) => self.deserialize_map(visitor)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_bool()?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as i8;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as i16;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
            let value = self.get_number()? as i32;
            visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as i64;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as u8;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as u16;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as u32;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as u64;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()? as f32;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_number()?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let string = self.get_string()?;
        let mut chars = string.chars();
        if let (Some(char), None) = (chars.next(), chars.next()){
            visitor.visit_char(char)
        } else {
            Err(PackageDeserializerError { cause: "Not a char".to_owned() })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_string()?;
        visitor.visit_str(&value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_string()?;
        visitor.visit_string(value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_bytes()?;
        visitor.visit_bytes(&value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_bytes()?;
        visitor.visit_byte_buf(value)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        if self.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.get_empty()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let value = self.get_string()?;
        if name == value {
            visitor.visit_unit()
        } else {
            Err(PackageDeserializerError { cause: format!("Expect '{name}' but found '{value}'") })
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let mut value = self.get_object()?;
        if value.len() == 1  {
            if let Some(package) = value.remove(name) {
                visitor.visit_newtype_struct(package)
            } else {
                Err(PackageDeserializerError { cause: format!("Object cannot be parsed into struct '{name}' because not have that entry") })
            }
        } else {
            Err(PackageDeserializerError { cause: format!("Object cannot be parsed into struct '{name}' because not have a unique entry") })
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_seq(DiscompoundArray::create(self, None, None)?)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_seq(DiscompoundArray::create(self, None, Some(len))?)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_seq(DiscompoundArray::create(self, Some(name), Some(len))?)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let object = self.get_object()?;
        visitor.visit_map(DiscompoundObject::create(object))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let object = self.get_object()?;
        visitor.visit_map(DiscompoundObject::create(object))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> 
    {
        let (variant, value) = match self {
            Package::Object(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(PackageDeserializerError { 
                            cause: "Expect object with a single key".to_owned()
                        });
                    }
                };
                // enums are encoded in json as maps with a single key:value pair
                if iter.next().is_some() {
                    return Err(PackageDeserializerError { 
                        cause: "Expect object with a single key".to_owned()
                    });
                }
                (variant, Some(value))
            },

            Package::String(variant) => (variant, None),

            _ => {
                return Err(PackageDeserializerError { 
                    cause: "Expect string or object".to_owned()
                });
            }
        };

        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_any(visitor)
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option<Package>,
}
impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = PackageDeserializerError;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer {
    value: Option<Package>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = PackageDeserializerError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(value) => Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de> {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => {
                Err(PackageDeserializerError { 
                    cause: "Expect type variant".to_owned()
                })
            }
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        match self.value {
            Some(package) => {
                visitor.visit_seq(DiscompoundArray::create(package, None, Some(len))?)
            }
            _ => {
                Err(PackageDeserializerError { 
                    cause: "Expect tuple variant".to_owned()
                })
            }
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        match self.value {
            Some(Package::Object(object)) => {
                visitor.visit_map(DiscompoundObject::create(object))
            }
            _ => {
                Err(PackageDeserializerError { 
                    cause: "Expect struct variant".to_owned()
                })
            }
        }
    }
}

struct DiscompoundArray {
    data: Vec<Package>
}
impl DiscompoundArray {
    pub fn create(package: Package, name: Option<&'static str>, len: Option<usize>) -> 
        Result<Self, PackageDeserializerError>
    {   
        let package = if let Some(name) = name {
            let mut value = package.get_object()?; 

            if value.len() == 1  {
                if let Some(package) = value.remove(name) {
                    package
                } else {
                    return Err(PackageDeserializerError { cause: format!("Object cannot be parsed into '{name}' because not have that entry") })
                }
            } else {
                return Err(PackageDeserializerError { cause: format!("Object cannot be parsed into '{name}' because not have a unique entry") })
            }
        } else {
            package
        };


        let mut package = package.get_array()?;
        if let Some(len) = len {
            if len != package.len() {
                return Err(PackageDeserializerError { 
                    cause: format!("Required a array with length '{len}', but found '{}'", package.len() )
                })
            }
        }

        package.reverse();
        Ok(
            Self { data: package }
        )
    }
}


impl<'de> SeqAccess<'de> for DiscompoundArray {
    type Error = PackageDeserializerError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> 
    {
        if let Some(package) = self.data.pop() {
            let value = seed.deserialize(package)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

struct DiscompoundObject {
    data: IntoIter<String, Package>,
    last: Option<Package>
}

impl DiscompoundObject {
    pub fn create(object: HashMap<String, Package>) -> Self {
        Self { data: object.into_iter(), last: None }
    }
}
impl<'de> MapAccess<'de> for DiscompoundObject {
    type Error = PackageDeserializerError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de> {
        match self.data.next() {
            Some((key, value)) => {
                let key = seed.deserialize(MapKeyDeserializer { key })?;
                self.last = Some(value);

                Ok(Some(key))
            },
            None => Ok(None),
        }
        
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de> {
        match self.last.take() {
            Some(value) => seed.deserialize(value),
            None => Err(PackageDeserializerError { 
                cause: "Value is missing".to_owned()
            }),
        }
    }
}
// endregion

// region: impl MapKeyDeserializer

struct MapKeyDeserializer {
    key: String
}

impl<'de> Deserializer<'de> for MapKeyDeserializer {
    type Error = PackageDeserializerError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })   
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<bool>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_bool(key)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<i8>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_i8(key)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<i16>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_i16(key)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<i32>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_i32(key)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<i64>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_i64(key)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<u8>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_u8(key)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<u16>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_u16(key)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<u32>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_u32(key)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<u64>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_u64(key)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<f32>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_f32(key)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<f64>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_f64(key)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let key = self.key.parse::<char>().map_err(|e| 
            PackageDeserializerError { cause: e.to_string() 
        })?;
        visitor.visit_char(key)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_str(&self.key)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_string(self.key)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        if self.key == "" {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        if self.key == "" {
            visitor.visit_unit()
        } else {
            Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        if self.key == name {
            visitor.visit_unit()
        } else {
            Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(PackageDeserializerError { cause: "Key could not be parsed".to_owned() })
    }
}
// endregion