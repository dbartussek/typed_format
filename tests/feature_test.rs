pub mod round_trip;

use maplit::*;
use serde_derive::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
enum Test {
    Foo(usize),
    Bar { bar: usize },
    Baz,
    Map(HashMap<usize, usize>),
    Bytes(Vec<u8>),
    String(String),
    Char(char),
    Option(Option<()>),
}

#[test]
fn value_round_trip() {
    let values = vec![
        Test::Foo(1),
        Test::Bar { bar: 2 },
        Test::Baz,
        Test::Map(hashmap! { 1 => 42, 2 => 5, 0 => 64}),
        Test::Bytes("Hello World".as_bytes().to_vec()),
        Test::String("\tHello World\"\"\\Me!\n\0".to_string()),
        Test::Char(' '),
        Test::Char('\n'),
        Test::Char('\\'),
        Test::Option(None),
        Test::Option(Some(())),
    ];

    for it in &values {
        round_trip::all_asserts(it);
    }

    round_trip::all_asserts(&values);
}
