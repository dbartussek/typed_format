use crate::value::parser::{parse_main_type, parse_main_type_identifier};
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Identifier(pub String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<&str> for Identifier {
    fn from(v: &str) -> Self {
        Identifier(v.to_string())
    }
}

impl<T> PartialEq<T> for Identifier
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other.as_ref())
    }
}


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GenericIdentifier {
    pub identifier: Identifier,
    pub generics: Option<Generics>,
}

impl Display for GenericIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.identifier, f)?;

        if let Some(generics) = self.generics.as_ref() {
            write!(f, "<{}>", generics)?;
        }

        Ok(())
    }
}

impl From<&str> for GenericIdentifier {
    fn from(v: &str) -> Self {
        GenericIdentifier {
            identifier: v.into(),
            generics: None,
        }
    }
}


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TypeIdentifier {
    pub segments: Vec<GenericIdentifier>,
}

impl TypeIdentifier {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        parse_main_type_identifier(input)
    }
}

impl Display for TypeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        display_with_separator(f, &self.segments, "::")
    }
}

impl From<&str> for TypeIdentifier {
    fn from(v: &str) -> Self {
        TypeIdentifier {
            segments: vec![v.into()],
        }
    }
}
impl From<(&str, &str)> for TypeIdentifier {
    fn from((a, b): (&str, &str)) -> Self {
        TypeIdentifier {
            segments: vec![a.into(), b.into()],
        }
    }
}


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generics {
    pub types: Vec<Type>,
}

impl Display for Generics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        display_with_separator(f, &self.types, ", ")
    }
}


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Type {
    TypeIdentifier(TypeIdentifier),
    Array { content: Box<Type>, size: String },
    Tuple(Vec<Type>),
}

impl Type {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        parse_main_type(input)
    }

    /// If this is a TypeIdentifier, get the last segment
    ///
    /// Used for serde deserialize
    pub fn get_identifier(&self) -> Option<&Identifier> {
        match self {
            Type::TypeIdentifier(identifier) => {
                identifier.segments.last().map(|i| &i.identifier)
            },
            _ => None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::TypeIdentifier(ident) => Display::fmt(ident, f),
            Type::Array { content, size } => {
                write!(f, "[{} ; {}]", content, size)
            },
            Type::Tuple(tuple) => {
                write!(f, "(")?;
                display_with_separator(f, &tuple, ", ")?;
                write!(f, ")")
            },
        }
    }
}

impl From<&str> for Type {
    fn from(v: &str) -> Self {
        Self::TypeIdentifier(TypeIdentifier::from(v))
    }
}
impl From<(&str, &str)> for Type {
    fn from(v: (&str, &str)) -> Self {
        Self::TypeIdentifier(TypeIdentifier::from(v))
    }
}


/// Utility to insert a separator between each item, but not at the end
fn display_with_separator<A, B>(
    f: &mut Formatter<'_>,
    items: &[A],
    separator: &B,
) -> std::fmt::Result
where
    A: Display,
    B: Display + ?Sized,
{
    if !items.is_empty() {
        for it in &items[..(items.len() - 1)] {
            Display::fmt(it, f)?;
            Display::fmt(separator, f)?;
        }

        Display::fmt(items.last().unwrap(), f)?;
    }

    Ok(())
}
