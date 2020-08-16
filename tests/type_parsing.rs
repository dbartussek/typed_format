use std::{any::type_name, borrow::Cow};
use typed_format::value::types::Type;

fn parse_type<T>()
where
    T: ?Sized,
{
    let name = type_name::<T>();
    println!("Type name: '{}'", name);
    let t = Type::parse(name).unwrap();
    println!("Parsed as {:#?}", &t);
}

#[test]
pub fn parse_usize() {
    parse_type::<usize>();
}

#[test]
pub fn parse_u64() {
    parse_type::<u64>();
}

#[test]
pub fn parse_struct() {
    struct Test;

    parse_type::<Test>();
}

#[test]
pub fn parse_vec() {
    parse_type::<Vec<usize>>();
}

#[test]
pub fn parse_vec_of_array() {
    parse_type::<Vec<[usize; 42]>>();
}

#[test]
pub fn parse_vec_of_tuple() {
    parse_type::<Vec<(usize, usize)>>();
}

#[test]
pub fn parse_array() {
    parse_type::<[usize; 42]>();
}

#[test]
pub fn parse_nested_array() {
    parse_type::<[[usize; 1]; 42]>();
}

#[test]
pub fn parse_tuple() {
    parse_type::<(usize, usize)>();
}

#[test]
pub fn parse_lifetime() {
    parse_type::<Cow<'static, str>>();
}
