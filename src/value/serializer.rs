use crate::value::{Identifier, TypeIdentifier, Value};
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
        SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

pub struct ValueSerializer;

#[derive(Debug)]
pub enum ValueSerializerError {
    Custom(String),
}

impl Display for ValueSerializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSerializerError::Custom(custom) => Display::fmt(custom, f),
        }
    }
}

impl std::error::Error for ValueSerializerError {}

impl serde::ser::Error for ValueSerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ValueSerializerError::Custom(msg.to_string())
    }
}

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = ValueSerializerError;
    type SerializeSeq = ValueSerializerSeq;
    type SerializeTuple = ValueSerializerSeq;
    type SerializeTupleStruct = ValueSerializerTupleStruct;
    type SerializeTupleVariant = ValueSerializerTupleStruct;
    type SerializeMap = ValueSerializerMap;
    type SerializeStruct = ValueSerializerStruct;
    type SerializeStructVariant = ValueSerializerStruct;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&base64::encode(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Option(None))
    }

    fn serialize_some<T: ?Sized>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let inner = value.serialize(ValueSerializer)?;
        Ok(Value::Option(Some(Box::new(inner))))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Type(name.into()))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Type((name, variant).into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let inner = value.serialize(ValueSerializer)?;
        Ok(Value::TupleStruct(name.into(), vec![inner]))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let inner = value.serialize(ValueSerializer)?;
        let identifier = (name, variant).into();
        Ok(Value::TupleStruct(identifier, vec![inner]))
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ValueSerializerSeq {
            items: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ValueSerializerSeq {
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ValueSerializerTupleStruct {
            identifier: name.into(),
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(ValueSerializerTupleStruct {
            identifier: (name, variant).into(),
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_map(
        self,
        _: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Ok(ValueSerializerMap {
            items: BTreeMap::new(),
            current_key: None,
            current_value: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(ValueSerializerStruct {
            identifier: name.into(),
            items: Default::default(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(ValueSerializerStruct {
            identifier: (name, variant).into(),
            items: Default::default(),
        })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        self.serialize_str(&value.to_string())
    }
}

pub struct ValueSerializerSeq {
    items: Vec<Value>,
}

impl SerializeSeq for ValueSerializerSeq {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::List(self.items))
    }
}

impl SerializeTuple for ValueSerializerSeq {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.items))
    }
}

pub struct ValueSerializerTupleStruct {
    identifier: TypeIdentifier,
    items: Vec<Value>,
}

impl SerializeTupleStruct for ValueSerializerTupleStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.items.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(self.identifier, self.items))
    }
}

impl SerializeTupleVariant for ValueSerializerTupleStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeTupleStruct::serialize_field(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeTupleStruct::end(self)
    }
}

pub struct ValueSerializerStruct {
    identifier: TypeIdentifier,
    items: BTreeMap<Identifier, Value>,
}

impl SerializeStruct for ValueSerializerStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let inner = value.serialize(ValueSerializer)?;
        self.items.insert(key.into(), inner);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.identifier, self.items))
    }
}

impl SerializeStructVariant for ValueSerializerStruct {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeStruct::end(self)
    }
}

pub struct ValueSerializerMap {
    items: BTreeMap<Value, Value>,

    current_key: Option<Value>,
    current_value: Option<Value>,
}

impl SerializeMap for ValueSerializerMap {
    type Ok = Value;
    type Error = ValueSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = key.serialize(ValueSerializer)?;

        match self.current_value.take() {
            Some(value) => {
                self.items.insert(key, value);
            },
            None => {
                self.current_key = Some(key);
            },
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = value.serialize(ValueSerializer)?;

        match self.current_key.take() {
            Some(key) => {
                self.items.insert(key, value);
            },
            None => {
                self.current_value = Some(value);
            },
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.current_key.is_some() {
            unimplemented!()
        }
        if self.current_value.is_some() {
            unimplemented!()
        }

        Ok(Value::Map(self.items))
    }
}
