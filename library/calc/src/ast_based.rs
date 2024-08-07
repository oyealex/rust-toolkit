use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map_res;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

#[derive(Debug, Clone)]
enum Expr {
    Num(Num),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    // Abs(Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
enum Num {
    Integer(i128),
    Double(f64),
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer + rhs_integer),
                Num::Double(rhs_double) => Num::Double(lhs_integer as f64 + rhs_double),
            },
            Num::Double(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Double(lhs_double + rhs_integer as f64),
                Num::Double(rhs_double) => Num::Double(lhs_double + rhs_double),
            },
        }
    }
}

impl Sub for Num {
    type Output = Num;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer - rhs_integer),
                Num::Double(rhs_double) => Num::Double(lhs_integer as f64 - rhs_double),
            },
            Num::Double(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Double(lhs_double - rhs_integer as f64),
                Num::Double(rhs_double) => Num::Double(lhs_double - rhs_double),
            },
        }
    }
}

impl Mul for Num {
    type Output = Num;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer * rhs_integer),
                Num::Double(rhs_double) => Num::Double(lhs_integer as f64 * rhs_double),
            },
            Num::Double(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Double(lhs_double * rhs_integer as f64),
                Num::Double(rhs_double) => Num::Double(lhs_double * rhs_double),
            },
        }
    }
}

impl Div for Num {
    type Output = Num;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer / rhs_integer),
                Num::Double(rhs_double) => Num::Double(lhs_integer as f64 / rhs_double),
            },
            Num::Double(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Double(lhs_double / rhs_integer as f64),
                Num::Double(rhs_double) => Num::Double(lhs_double / rhs_double),
            },
        }
    }
}

impl Expr {
    fn eval(&self) -> Num {
        match self {
            Expr::Num(num) => num.clone(),
            Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expr::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
            // Expr::Abs(ohs) => todo!(),
        }
    }
}

fn parse_num(input: &str) -> IResult<&str, Expr> {
    // FIXME: handle +-
    map_res(delimited(space0, digit1, space0), |digit_str: &str| {
        i128::from_str(digit_str).map(|value: i128| Expr::Num(Num::Integer(value)))
    })(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        delimited(space0, char('('), space0),
        parse_expr,
        delimited(space0, char(')'), space0),
    )(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((parse_num, parse_parens))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    let mut parse = fold_many0(
        pair(alt((tag("*"), tag("/"))), parse_factor),
        || init.clone(),
        |acc_expr: Expr, (op, expr): (&str, Expr)| match op {
            "*" => Expr::Mul(Box::new(acc_expr), Box::new(expr)),
            "/" => Expr::Div(Box::new(acc_expr), Box::new(expr)),
            other => {
                panic!("unexpected operator: {}", other)
            }
        },
    );
    parse(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    let mut parse = fold_many0(
        pair(alt((tag("+"), tag("-"))), parse_term),
        || init.clone(),
        |acc_expr: Expr, (op, expr): (&str, Expr)| match op {
            "+" => Expr::Add(Box::new(acc_expr), Box::new(expr)),
            "-" => Expr::Sub(Box::new(acc_expr), Box::new(expr)),
            other => {
                panic!("unexpected operator: {}", other)
            }
        },
    );
    parse(input)
}

#[cfg(test)]
mod test {
    use crate::ast_based::parse_expr;

    #[test]
    fn test() {
        // let input = "8 + 2 - 4 * 5 / 2";
        let input = "3 + 5 * (2 - 8) / 4";
        match parse_expr(input) {
            Ok((_, expr)) => {
                println!("{:?}", expr);
                println!("{:?}", expr.eval())
            }
            Err(err) => println!("{:?}", err),
        }
    }
}
