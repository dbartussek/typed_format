use crate::value::{
    types::{GenericIdentifier, Identifier, Type, TypeIdentifier},
    Value,
};
use anyhow::anyhow;
use pest::{iterators::Pair, Parser};
use pest_derive::*;
use std::{collections::BTreeMap, str::Chars};

#[derive(Parser)]
#[grammar = "value/value.pest"]
struct ValueParser;

fn parse_identifier(pair: Pair<Rule>) -> anyhow::Result<Identifier> {
    assert_eq!(pair.as_rule(), Rule::identifier);
    Ok(Identifier(pair.as_str().to_string()))
}

fn parse_type_identifier(pair: Pair<Rule>) -> anyhow::Result<TypeIdentifier> {
    fn parse_generic_identifier(
        pair: Pair<Rule>,
    ) -> anyhow::Result<GenericIdentifier> {
        assert_eq!(pair.as_rule(), Rule::generic_identifier);

        let mut pairs = pair.into_inner();

        let identifier = parse_identifier(pairs.next().unwrap())?;

        if let Some(generic) = pairs.next() {
            return Err(anyhow!(
                "Generic identifiers are not supported {:?}",
                generic
            ));
        }

        Ok(GenericIdentifier {
            identifier,
            generics: None,
        })
    }

    let segments = pair
        .into_inner()
        .map(parse_generic_identifier)
        .collect::<anyhow::Result<Vec<GenericIdentifier>>>()?;

    Ok(TypeIdentifier { segments })
}

fn parse_generic_type(pair: Pair<Rule>) -> anyhow::Result<Type> {
    assert_eq!(pair.as_rule(), Rule::generic_type);

    let inner = pair.into_inner().next().unwrap();

    Ok(match inner.as_rule() {
        Rule::type_identifier => {
            Type::TypeIdentifier(parse_type_identifier(inner)?)
        },
        _ => return Err(anyhow!("Unknown type: {:#?}", &inner)),
    })
}

fn parse_tuple_inner(pair: Pair<Rule>) -> anyhow::Result<Vec<Value>> {
    assert!(pair.as_rule() == Rule::tuple || pair.as_rule() == Rule::list);

    pair.into_inner().map(parse_value).collect()
}

fn parse_tuple(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::tuple);

    Ok(Value::Tuple(parse_tuple_inner(pair)?))
}

fn parse_list(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::list);

    Ok(Value::List(parse_tuple_inner(pair)?))
}

fn parse_tuple_struct(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::tuple_struct);

    let mut pairs = pair.into_inner();

    let identifier = parse_type_identifier(pairs.next().unwrap())?;
    let tuple = parse_tuple_inner(pairs.next().unwrap())?;

    Ok(Value::TupleStruct(identifier, tuple))
}

fn parse_named_struct(pair: Pair<Rule>) -> anyhow::Result<Value> {
    fn parse_named_tuple_entry(
        pair: Pair<Rule>,
    ) -> anyhow::Result<(Identifier, Value)> {
        assert_eq!(pair.as_rule(), Rule::named_tuple_entry);

        let mut pairs = pair.into_inner();

        let identifier = parse_identifier(pairs.next().unwrap())?;
        let value = parse_value(pairs.next().unwrap())?;

        Ok((identifier, value))
    }

    fn parse_named_tuple(
        pair: Pair<Rule>,
    ) -> anyhow::Result<BTreeMap<Identifier, Value>> {
        assert_eq!(pair.as_rule(), Rule::named_tuple);

        pair.into_inner().map(parse_named_tuple_entry).collect()
    }

    assert_eq!(pair.as_rule(), Rule::named_struct);

    let mut pairs = pair.into_inner();

    let identifier = parse_type_identifier(pairs.next().unwrap())?;
    let fields = parse_named_tuple(pairs.next().unwrap())?;

    Ok(Value::Struct(identifier, fields))
}

fn parse_map(pair: Pair<Rule>) -> anyhow::Result<Value> {
    fn parse_map_entry(pair: Pair<Rule>) -> anyhow::Result<(Value, Value)> {
        assert_eq!(pair.as_rule(), Rule::map_entry);

        let mut pairs = pair.into_inner();

        let key = parse_value(pairs.next().unwrap())?;
        let value = parse_value(pairs.next().unwrap())?;

        Ok((key, value))
    }

    assert_eq!(pair.as_rule(), Rule::map);

    let map = pair
        .into_inner()
        .map(parse_map_entry)
        .collect::<anyhow::Result<BTreeMap<Value, Value>>>()?;
    Ok(Value::Map(map))
}

fn parse_number(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::number);

    Ok(Value::Number(pair.as_str().to_string()))
}

/// Consumes input until a single char can be unescaped, if necessary
fn unescape_single(chars: &mut Chars) -> anyhow::Result<char> {
    let c = chars.next().unwrap();

    if c != '\\' {
        Ok(c)
    } else {
        let next = match chars.next() {
            Some(c) => c,
            None => {
                return Err(anyhow!(
                    "Unexpected end of string in escape sequence"
                ))
            },
        };

        Ok(match next {
            '\\' => '\\',

            'n' => '\n',
            'r' => '\r',
            't' => '\t',

            '0' => '\0',

            '"' => '"',
            '\'' => '\'',

            other => {
                return Err(anyhow!("Unknown escape character {:?}", other))
            },
        })
    }
}

fn parse_string(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::string);

    let raw_string = pair.into_inner().next().unwrap().as_str();

    fn unescape_string(input: &str) -> anyhow::Result<String> {
        let mut chars = input.chars();

        Ok(std::iter::from_fn(|| {
            if chars.as_str().is_empty() {
                None
            } else {
                Some(unescape_single(&mut chars))
            }
        })
        .collect::<anyhow::Result<String>>()?)
    }

    Ok(Value::String(unescape_string(raw_string)?))
}
fn parse_char(pair: Pair<Rule>) -> anyhow::Result<Value> {
    assert_eq!(pair.as_rule(), Rule::value_char);

    let raw_string = pair.into_inner().next().unwrap().as_str();

    fn unescape_char(input: &str) -> anyhow::Result<char> {
        let mut chars = input.chars();

        let c = unescape_single(&mut chars)?;

        if !chars.as_str().is_empty() {
            return Err(anyhow!("Garbage at the end of char"));
        }

        Ok(c)
    }

    Ok(Value::Char(unescape_char(raw_string)?))
}

fn parse_value(pair: Pair<Rule>) -> anyhow::Result<Value> {
    match pair.as_rule() {
        Rule::unit => Ok(Value::Unit),

        Rule::number => parse_number(pair),
        Rule::string => parse_string(pair),
        Rule::value_char => parse_char(pair),

        Rule::none => Ok(Value::Option(None)),
        Rule::some => Ok(Value::Option(Some(Box::new(parse_value(
            pair.into_inner().next().unwrap(),
        )?)))),

        Rule::tuple => parse_tuple(pair),
        Rule::list => parse_list(pair),

        Rule::tuple_struct => parse_tuple_struct(pair),
        Rule::named_struct => parse_named_struct(pair),
        Rule::map => parse_map(pair),

        Rule::generic_type => Ok(Value::Type(parse_generic_type(pair)?)),

        _ => panic!("Unknown value {:#?}", pair),
    }
}

/// Utility function to parse a string into a value
fn parse_starter<F, T>(
    input: &str,
    rule: Rule,
    function: F,
) -> anyhow::Result<T>
where
    F: FnOnce(Pair<Rule>) -> anyhow::Result<T>,
{
    let mut raw = ValueParser::parse(rule, input)?;
    let pair = raw.next().unwrap();
    function(pair)
}

pub fn parse_main_value(input: &str) -> anyhow::Result<Value> {
    parse_starter(input, Rule::main_value, parse_value)
}

pub fn parse_main_type_identifier(
    input: &str,
) -> anyhow::Result<TypeIdentifier> {
    parse_starter(input, Rule::main_type_identifier, parse_type_identifier)
}

pub fn parse_main_type(input: &str) -> anyhow::Result<Type> {
    parse_starter(input, Rule::main_type, parse_generic_type)
}
