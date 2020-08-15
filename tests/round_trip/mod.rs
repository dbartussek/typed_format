use serde::{de::DeserializeOwned, export::fmt::Debug, Serialize};
use typed_format::value::Value;

/// Converts T to Value and back, then checks if they are equal
pub fn assert_value<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let value = Value::new(t);

    let round_trip: T = value.deserialize().unwrap();

    assert_eq!(*t, round_trip);
}

/// Serializes T to String and back, then checks if they are equal
pub fn assert_string_pretty<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let value = Value::new(t);
    let _string = value.to_string_pretty();
}

/// Serializes T to String and back, then checks if they are equal
pub fn assert_string_compact<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let value = Value::new(t);
    let _string = value.to_string_compact();
}

pub fn all_asserts<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_value(t);
    assert_string_compact(t);
    assert_string_pretty(t);
}
