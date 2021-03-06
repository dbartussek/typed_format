use serde::{de::DeserializeOwned, export::fmt::Debug, Serialize};
use std::path::Path;
use typed_format::value::Value;

pub fn test_output_path() -> &'static Path {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_output"));
    let _ = std::fs::create_dir_all(path);
    path
}

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
    let string = value.to_string_pretty();

    assert_parse(t, &value, &string);
}

/// Serializes T to String and back, then checks if they are equal
pub fn assert_string_compact<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let value = Value::new(t);
    let string = value.to_string_compact();

    assert_parse(t, &value, &string);
}

fn assert_parse<T>(t: &T, value: &Value, string: &str)
where
    T: DeserializeOwned + PartialEq + Debug,
{
    std::fs::write(test_output_path().join("parse.tyf"), string).unwrap();

    let parsed_value = Value::parse(&string).unwrap();

    assert_eq!(*value, parsed_value);

    let deserialized: T = parsed_value.deserialize().unwrap();

    assert_eq!(*t, deserialized);
}

pub fn all_asserts<T>(t: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_value(t);
    assert_string_compact(t);
    assert_string_pretty(t);
}
