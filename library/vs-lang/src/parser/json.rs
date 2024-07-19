#![allow(unused)]

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_while};
use nom::character::complete::{alphanumeric1, anychar, char, multispace0, one_of};
use nom::combinator::{cut, map, opt, value};
use nom::error::context;
use nom::multi::separated_list0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{IResult, Parser};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn interspace(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn null(input: &str) -> IResult<&str, ()> {
    context("num", value((), tag("null"))).parse(input)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    escaped(take_while(|c| c != '\\'), '\\', one_of("\"nrt\\"))(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    context(
        "string",
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
    )
    .parse(input)
}

fn boolean(input: &str) -> IResult<&str, bool> {
    context(
        "boolean",
        alt((value(true, tag("true")), value(false, tag("false")))),
    )
    .parse(input)
}

fn num(input: &str) -> IResult<&str, f64> {
    context("num", double).parse(input)
}

fn array(input: &str) -> IResult<&str, Vec<JsonValue>> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(interspace, char(',')), json_value),
                preceded(interspace, char(']')),
            )),
        ),
    )
    .parse(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, JsonValue)> {
    separated_pair(
        preceded(interspace, string),
        cut(preceded(interspace, char(':'))),
        json_value,
    )
    .parse(input)
}

fn hash(input: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(interspace, char(',')), key_value),
                    |tuples| {
                        tuples
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(interspace, char('}')),
            )),
        ),
    )
    .parse(input)
}

fn json_value(input: &str) -> IResult<&str, JsonValue> {
    preceded(
        interspace,
        alt((
            map(null, |_| JsonValue::Null),
            map(string, |s| JsonValue::Str(String::from(s))),
            map(boolean, JsonValue::Boolean),
            map(num, JsonValue::Num),
            map(array, JsonValue::Array),
            map(hash, JsonValue::Object),
        )),
    )
    .parse(input)
}

fn root(input: &str) -> IResult<&str, JsonValue> {
    delimited(
        interspace,
        alt((
            map(hash, JsonValue::Object),
            map(array, JsonValue::Array),
            map(null, |_| JsonValue::Null),
        )),
        opt(interspace),
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::parser::json::{null, string};

    #[test]
    fn test_null() {
        assert_eq!(Ok(("", ())), null("null"));
    }

    #[test]
    fn test_string() {
        assert_eq!(Ok(("", "This is a string")), string("\"This is a string\""));
    }
}
