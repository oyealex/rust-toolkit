use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt, recognize};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::cmp::Ordering;

#[derive(Debug)]
enum Number {
    Integer(i128),
    Double(f64),
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    // LeftParenthesis,
    // RightParenthesis,
}

impl Operator {
    fn priority(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl Eq for Operator {}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority().cmp(&other.priority()))
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

#[derive(Debug)]
struct Calculator {
    numbers: Vec<Number>,
    operators: Vec<Operator>,
}

impl Calculator {
    fn append_number(&mut self, number: Number) {
        self.numbers.push(number);
    }

    fn append_operator(&mut self, operator: Operator) {
        self.operators.push(operator);
    }

    fn get_result(&self) -> Option<&Number> {
        self.numbers.first()
    }
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
    use crate::calculator::Operator;
    use crate::calculator::{num, operator};

    #[test]
    fn test_num_parse() {
        assert_eq!(Ok(("", 123)), num("123"));
        assert_eq!(Ok(("", 123)), num("+123"));
        assert_eq!(Ok(("", -123)), num("-123"));
    }

    #[test]
    fn test_operator_parse() {
        assert_eq!(Ok(("", Operator::Add)), operator("+"));
        assert_eq!(Ok(("", Operator::Subtract)), operator("-"));
        assert_eq!(Ok(("", Operator::Multiply)), operator("*"));
        assert_eq!(Ok(("", Operator::Divide)), operator("/"));
    }

    #[test]
    fn test_operator_compare() {
        assert_eq!(Operator::Add, Operator::Subtract);
        assert_eq!(Operator::Multiply, Operator::Divide);
        assert!(Operator::Add < Operator::Multiply);
        assert!(Operator::Subtract < Operator::Multiply);
        assert!(Operator::Add < Operator::Divide);
        assert!(Operator::Subtract < Operator::Divide);
    }
}
