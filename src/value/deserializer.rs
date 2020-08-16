use crate::value::{
    types::{GenericIdentifier, Identifier, Type, TypeIdentifier},
    Value,
};
use anyhow::{anyhow, Context, Error};
use serde::{
    de::{
        DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserializer,
};
use std::{
    collections::btree_map,
    fmt::{Display, Formatter},
};

pub struct ValueDeserializer<'value> {
    pub value: &'value Value,
}

#[derive(Debug)]
pub struct ValueDeserializerError(pub anyhow::Error);

impl From<anyhow::Error> for ValueDeserializerError {
    fn from(e: Error) -> Self {
        ValueDeserializerError(e)
    }
}

impl Display for ValueDeserializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for ValueDeserializerError {}

impl serde::de::Error for ValueDeserializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ValueDeserializerError(anyhow!("{}", msg))
    }
}

macro_rules! number_body {
    ($this:expr, $visitor:expr, $visit_function:ident) => {{
        expect_deserialize!($this, Value::Number(__v), {
            let __v = __v.parse().with_context(|| {
                format!("Failed to parse {:?} as number", ($this).value)
            })?;
            Ok(($visitor).$visit_function::<ValueDeserializerError>(__v)?)
        })
    }};
}

macro_rules! expect_deserialize {
    ($this:expr, $pattern:pat, $match_arm:expr) => {{
        match ($this).value {
            $pattern => $match_arm,
            _ => Err(ValueDeserializerError(anyhow!(
                "Expected {}, found {:?}",
                stringify!($pattern),
                ($this).value,
            ))),
        }
    }};
}

impl<'value, 'de> Deserializer<'de> for ValueDeserializer<'value> {
    type Error = ValueDeserializerError;

    fn deserialize_any<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Unit => self.deserialize_unit(visitor),
            Value::Bool(_) => self.deserialize_bool(visitor),
            Value::Char(_) => self.deserialize_char(visitor),
            Value::String(_) => self.deserialize_str(visitor),
            Value::Number(_) => unimplemented!(),
            Value::Type(_) => unimplemented!(),
            Value::List(_) => unimplemented!(),
            Value::Tuple(_) => unimplemented!(),
            Value::Map(_) => unimplemented!(),
            Value::Option(_) => unimplemented!(),
            Value::Struct(_, _) => unimplemented!(),
            Value::TupleStruct(_, _) => unimplemented!(),
        }
    }

    fn deserialize_bool<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(v) => {
                Ok(visitor.visit_bool::<ValueDeserializerError>(*v)?)
            },
            _ => Err(ValueDeserializerError(anyhow!(
                "Expected Bool, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_i8<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_i8)
    }

    fn deserialize_i16<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_i16)
    }

    fn deserialize_i32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_i32)
    }

    fn deserialize_i64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_i64)
    }

    fn deserialize_u8<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_u8)
    }

    fn deserialize_u16<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_u16)
    }

    fn deserialize_u32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_u32)
    }

    fn deserialize_u64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_u64)
    }

    fn deserialize_f32<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_f32)
    }

    fn deserialize_f64<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        number_body!(self, visitor, visit_f64)
    }

    fn deserialize_char<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(
            self,
            Value::Char(c),
            visitor.visit_char::<ValueDeserializerError>(*c)
        )
    }

    fn deserialize_str<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(
            self,
            Value::String(s),
            visitor.visit_str::<ValueDeserializerError>(&s)
        )
    }

    fn deserialize_string<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::deserialize_str(self, visitor)
    }

    fn deserialize_bytes<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::String(s), {
            let b = base64::decode(&s).with_context(|| {
                format!("Could not decode as base 64: {:?}", self.value)
            })?;

            visitor.visit_bytes::<ValueDeserializerError>(&b)
        })
    }

    fn deserialize_byte_buf<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::deserialize_bytes(self, visitor)
    }

    fn deserialize_option<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::Option(v), {
            match v {
                Some(value) => visitor.visit_some(ValueDeserializer { value }),
                None => visitor.visit_none::<ValueDeserializerError>(),
            }
        })
    }

    fn deserialize_unit<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::Unit, {
            visitor.visit_unit::<ValueDeserializerError>()
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple_struct(name, 0, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple_struct(name, 1, visitor)
    }

    fn deserialize_seq<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::List(seq), {
            visitor.visit_seq(ValueDeserializerSequence(&seq))
        })
    }

    fn deserialize_tuple<V>(
        self,
        _: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::Tuple(seq), {
            visitor.visit_seq(ValueDeserializerSequence(&seq))
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::TupleStruct(_, seq), {
            visitor.visit_seq(ValueDeserializerSequence(&seq))
        })
    }

    fn deserialize_map<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::Map(map), {
            visitor.visit_map(ValueDeserializerMap {
                iter: map.iter(),
                current_value: None,
                current_key: None,
            })
        })
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        expect_deserialize!(self, Value::Struct(_, map), {
            visitor.visit_map(ValueDeserializerStruct {
                iter: map.iter(),
                current_value: None,
                current_key: None,
            })
        })
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(ValueDeserializerEnum { value: self.value })
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::TupleStruct(identifier, _)
            | Value::Struct(identifier, _) => {
                match identifier.segments.last() {
                    Some(identifier) => {
                        return visitor
                            .visit_str(identifier.identifier.0.as_str())
                    },
                    _ => {},
                }
            },
            Value::Type(t) => match t {
                Type::TypeIdentifier(identifier) => {
                    match identifier.segments.last() {
                        Some(identifier) => {
                            return visitor
                                .visit_str(identifier.identifier.0.as_str())
                        },
                        _ => {},
                    }
                },
                _ => {},
            },
            _ => {},
        }

        Err(anyhow!("{:?} is not Identifier", self.value).into())
    }

    fn deserialize_ignored_any<V>(
        self,
        _: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct ValueDeserializerSequence<'lt>(&'lt [Value]);

impl<'lt, 'de> SeqAccess<'de> for ValueDeserializerSequence<'lt> {
    type Error = ValueDeserializerError;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.first() {
            None => Ok(None),
            Some(value) => {
                let result =
                    Ok(Some(seed.deserialize(ValueDeserializer { value })?));
                self.0 = &(self.0)[1..];
                result
            },
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

struct ValueDeserializerMap<'lt> {
    iter: btree_map::Iter<'lt, Value, Value>,
    current_key: Option<&'lt Value>,
    current_value: Option<&'lt Value>,
}

impl<'lt> ValueDeserializerMap<'lt> {
    fn next(&mut self) {
        if let Some((key, value)) = self.iter.next() {
            self.current_key = Some(key);
            self.current_value = Some(value);
        }
    }
}

impl<'lt, 'de> MapAccess<'de> for ValueDeserializerMap<'lt> {
    type Error = ValueDeserializerError;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.current_key.is_none() {
            self.next();
        }

        match self.current_key.take() {
            None => Ok(None),
            Some(value) => {
                Ok(Some(seed.deserialize(ValueDeserializer { value })?))
            },
        }
    }

    fn next_value_seed<V>(
        &mut self,
        seed: V,
    ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.current_value.is_none() {
            self.next();
        }

        match self.current_value.take() {
            None => {
                Err(anyhow!("Called next_value on empty map iterator").into())
            },
            Some(value) => seed.deserialize(ValueDeserializer { value }),
        }
    }
}

struct ValueDeserializerStruct<'lt> {
    iter: btree_map::Iter<'lt, Identifier, Value>,
    current_key: Option<&'lt Identifier>,
    current_value: Option<&'lt Value>,
}

impl<'lt> ValueDeserializerStruct<'lt> {
    fn next(&mut self) {
        if let Some((key, value)) = self.iter.next() {
            self.current_key = Some(key);
            self.current_value = Some(value);
        }
    }
}

impl<'lt, 'de> MapAccess<'de> for ValueDeserializerStruct<'lt> {
    type Error = ValueDeserializerError;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.current_key.is_none() {
            self.next();
        }

        match self.current_key.take() {
            None => Ok(None),
            Some(value) => {
                let value = Value::Type(Type::TypeIdentifier(TypeIdentifier {
                    segments: vec![GenericIdentifier {
                        identifier: value.clone(),
                        generics: None,
                    }],
                }));
                let value = &value;
                Ok(Some(seed.deserialize(ValueDeserializer { value })?))
            },
        }
    }

    fn next_value_seed<V>(
        &mut self,
        seed: V,
    ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.current_value.is_none() {
            self.next();
        }

        match self.current_value.take() {
            None => {
                Err(anyhow!("Called next_value on empty map iterator").into())
            },
            Some(value) => seed.deserialize(ValueDeserializer { value }),
        }
    }
}

struct ValueDeserializerEnum<'lt> {
    value: &'lt Value,
}

impl<'lt, 'de> EnumAccess<'de> for ValueDeserializerEnum<'lt> {
    type Error = ValueDeserializerError;
    type Variant = Self;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value =
            seed.deserialize(ValueDeserializer { value: self.value })?;
        Ok((value, self))
    }
}

impl<'lt, 'de> VariantAccess<'de> for ValueDeserializerEnum<'lt> {
    type Error = ValueDeserializerError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(
        self,
        seed: T,
    ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        expect_deserialize!(self, Value::TupleStruct(_, fields), {
            if fields.len() != 1 {
                return Err(anyhow!(
                    "Expected newtype variant {:?}",
                    self.value
                )
                .into());
            }

            seed.deserialize(ValueDeserializer {
                value: fields.get(0).unwrap(),
            })
        })
    }

    fn tuple_variant<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ValueDeserializer { value: self.value }
            .deserialize_tuple_struct("", len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ValueDeserializer { value: self.value }
            .deserialize_struct("", fields, visitor)
    }
}
