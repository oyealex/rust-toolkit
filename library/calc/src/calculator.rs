use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt, recognize};
use nom::sequence::tuple;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
}

fn num(input: &str) -> IResult<&str, i128> {
    map(
        recognize(tuple((opt(alt((char('+'), char('-')))), digit1))),
        |s: &str| s.parse::<i128>().unwrap(),
    )
    .parse(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(char('+'), |_| Operator::Add),
        map(char('-'), |_| Operator::Subtract),
        map(char('*'), |_| Operator::Multiply),
        map(char('/'), |_| Operator::Divide),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::calculator::{operator, num};
    use crate::calculator::Operator;

    #[test]
    fn test_num() {
        assert_eq!(Ok(("", 123)), num("123"));
        assert_eq!(Ok(("", 123)), num("+123"));
        assert_eq!(Ok(("", -123)), num("-123"));
    }

    #[test]
    fn test_operator() {
        assert_eq!(Ok(("", Operator::Add)), operator("+"));
        assert_eq!(Ok(("", Operator::Subtract)), operator("-"));
        assert_eq!(Ok(("", Operator::Multiply)), operator("*"));
        assert_eq!(Ok(("", Operator::Divide)), operator("/"));
    }
}
