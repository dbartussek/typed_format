pub mod deserializer;
pub(crate) mod parser;
pub mod printer;
pub mod serializer;
pub mod types;

use crate::value::{
    deserializer::{ValueDeserializer, ValueDeserializerError},
    printer::ValuePrinter,
    serializer::{ValueSerializer, ValueSerializerError},
    types::{Identifier, Type, TypeIdentifier},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Value {
    Unit,
    Bool(bool),
    Char(char),
    String(String),
    Number(String),

    Type(Type),

    List(Vec<Value>),
    Tuple(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Option(Option<Box<Value>>),

    Struct(TypeIdentifier, BTreeMap<Identifier, Value>),
    TupleStruct(TypeIdentifier, Vec<Value>),
}

impl Value {
    pub fn try_new<S>(s: S) -> Result<Value, ValueSerializerError>
    where
        S: Serialize,
    {
        s.serialize(ValueSerializer)
    }
    pub fn new<S>(s: S) -> Value
    where
        S: Serialize,
    {
        Self::try_new(s).unwrap()
    }

    pub fn parse(string: &str) -> anyhow::Result<Self> {
        parser::parse_main_value(string)
    }

    pub fn deserialize<'lt, T>(&'lt self) -> Result<T, ValueDeserializerError>
    where
        T: Deserialize<'lt>,
    {
        T::deserialize(ValueDeserializer { value: self })
    }

    pub fn to_string_pretty(&self) -> String {
        let mut buffer = String::new();
        let printer = ValuePrinter::pretty();

        printer.write(self, &mut buffer).unwrap();

        buffer
    }
    pub fn to_string_compact(&self) -> String {
        let mut buffer = String::new();
        let printer = ValuePrinter::compact();

        printer.write(self, &mut buffer).unwrap();

        buffer
    }
}
