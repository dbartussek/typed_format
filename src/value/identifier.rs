use std::fmt::{Display, Formatter};

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
pub enum TypeIdentifier {
    Struct(Identifier),
    Variant(Identifier, Identifier),
}

impl Display for TypeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeIdentifier::Struct(s) => Display::fmt(s, f),
            TypeIdentifier::Variant(s, v) => write!(f, "{}::{}", s, v),
        }
    }
}

impl From<&str> for TypeIdentifier {
    fn from(v: &str) -> Self {
        TypeIdentifier::Struct(v.into())
    }
}
impl From<(&str, &str)> for TypeIdentifier {
    fn from((a, b): (&str, &str)) -> Self {
        TypeIdentifier::Variant(a.into(), b.into())
    }
}

impl PartialEq<str> for TypeIdentifier {
    fn eq(&self, other: &str) -> bool {
        match self {
            TypeIdentifier::Struct(inner) => inner == &other,
            _ => false,
        }
    }
}

impl PartialEq<(&str, &str)> for TypeIdentifier {
    fn eq(&self, (a, b): &(&str, &str)) -> bool {
        match self {
            TypeIdentifier::Variant(left, right) => left == a && right == b,
            _ => false,
        }
    }
}
