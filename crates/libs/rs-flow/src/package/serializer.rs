use std::{collections::HashMap, fmt::Display};

use serde::{
    ser::{
        SerializeMap, 
        SerializeSeq, 
        SerializeStruct, 
        SerializeStructVariant, 
        SerializeTuple, 
        SerializeTupleStruct, 
        SerializeTupleVariant
    }, 
    Serializer
};

use super::Package;

#[derive(Debug)]
pub struct PackageSerializerError {
    pub cause: String
}

impl std::fmt::Display for PackageSerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::error::Error for PackageSerializerError {}

impl serde::ser::Error for PackageSerializerError {
    fn custom<T>(msg:T) -> Self 
        where T: Display 
    {
        PackageSerializerError { cause: msg.to_string() }
    }
}


// region: MapKeySerializer
struct MapKeySerializer;
struct Impossible; 

impl Serializer for MapKeySerializer {
    type Ok = String;
    type Error = PackageSerializerError;

    type SerializeSeq = Impossible;
    type SerializeTuple = Impossible;
    type SerializeTupleStruct = Impossible;
    type SerializeTupleVariant = Impossible;
    type SerializeMap = Impossible;
    type SerializeStruct = Impossible;
    type SerializeStructVariant = Impossible;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok("".to_string())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok("".to_string())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(name.to_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        Err(PackageSerializerError { cause: "Variant cannot be serialized into string".to_owned() })
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        Err(PackageSerializerError { cause: "Variant cannot be serialized into string".to_owned() })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(PackageSerializerError { cause: "Only string can be a key".to_owned() })
    }
} 

impl SerializeSeq for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeTuple for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeTupleStruct for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeTupleVariant for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeMap for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeStruct for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl SerializeStructVariant for Impossible {
    type Ok = String;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unreachable!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}


// endregion

// region: PackageSerializer
pub(crate) struct PackageSerializer;

pub(crate) struct CompoundArray {
    name: Option<String>,
    data: Vec<Package>
}
pub(crate) struct CompoundObjects {
    name: Option<String>,
    data: HashMap::<String, Package>,
}


impl Serializer for PackageSerializer {
    type Ok = Package;
    type Error = PackageSerializerError;

    type SerializeSeq = CompoundArray;
    type SerializeTuple = CompoundArray;
    type SerializeTupleStruct = CompoundArray;
    type SerializeTupleVariant = CompoundArray;
    type SerializeMap = CompoundObjects;
    type SerializeStruct = CompoundObjects;
    type SerializeStructVariant = CompoundObjects;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Package::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Empty)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Empty)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Package::String(name.to_string()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Empty)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(self)?;
        let key = name.to_string();
        Ok(Package::Object(HashMap::from([(key, value)])))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(self)?;
        let key = variant.to_string();
        Ok(Package::Object(HashMap::from([(key, value)])))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.unwrap_or(0);
        Ok(CompoundArray { name: None, data: Vec::with_capacity(len) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(CompoundArray { name: None, data: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(CompoundArray { name: None, data: Vec::with_capacity(len) })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(CompoundArray { name: Some(variant.to_string()), data: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.unwrap_or(0);
        Ok(CompoundObjects { name: None, data: HashMap::with_capacity(len) })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(CompoundObjects { name: None, data: HashMap::with_capacity(len) })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(CompoundObjects { name: Some(variant.to_string()), data: HashMap::with_capacity(len) })
    }
    
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }
    
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }
} 


impl SerializeSeq for CompoundArray {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Array(self.data))
    }
}

impl SerializeTuple for CompoundArray {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Array(self.data))
    }
}

impl SerializeTupleStruct for CompoundArray {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Array(self.data))
    }
}

impl SerializeTupleVariant for CompoundArray {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some(name) = self.name {
            let data = Package::Array(self.data);
            let data = HashMap::from([(name, data)]);
            Ok(Package::Object(data))
        } else {
            Ok(Package::Array(self.data))
        }
    }
}

impl SerializeMap for CompoundObjects {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_entry<K: ?Sized, V: ?Sized>(
            &mut self,
            key: &K,
            value: &V,
        ) -> Result<(), Self::Error>
        where
            K: serde::Serialize,
            V: serde::Serialize, 
    {
        let key = key.serialize(MapKeySerializer)?;
        let value = value.serialize(PackageSerializer)?;
        self.data.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Object(self.data))
    }
    
    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unimplemented!()
    }
    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        unimplemented!()
    }
}

impl SerializeStruct for CompoundObjects {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.insert(key.to_owned(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Package::Object(HashMap::from(self.data)))
    }
}

impl SerializeStructVariant for CompoundObjects {
    type Ok = Package;
    type Error = PackageSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        let value = value.serialize(PackageSerializer)?;
        self.data.insert(key.to_owned(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some(name) = self.name {
            let data = Package::Object(HashMap::from(self.data));
            let data = Package::Object(HashMap::from([(name, data)]));
            return Ok(data);
        }
        Ok(Package::Object(HashMap::from(self.data)))
    }
}

// endregion