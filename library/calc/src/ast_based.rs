use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{cut, map, map_res, opt, recognize};
use nom::error::{context, VerboseError};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

#[derive(Clone)]
enum Expr {
    Num(Num),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    ExactDiv(Box<Expr>, Box<Expr>),
    // Abs(Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pretty(f, 0)
    }
}

impl Expr {
    fn pretty(&self, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
        match self {
            Expr::Num(ohs) => writeln!(f, "{}── {}", "   │".repeat(level), ohs),
            Expr::Add(lhs, rhs) => {
                rhs.pretty(f, level + 1)?;
                writeln!(f, "{}── +", "   │".repeat(level))?;
                lhs.pretty(f, level + 1)
            }
            Expr::Sub(lhs, rhs) => {
                rhs.pretty(f, level + 1)?;
                writeln!(f, "{}── -", "   │".repeat(level))?;
                lhs.pretty(f, level + 1)
            }
            Expr::Mul(lhs, rhs) => {
                rhs.pretty(f, level + 1)?;
                writeln!(f, "{}── *", "   │".repeat(level))?;
                lhs.pretty(f, level + 1)
            }
            Expr::Div(lhs, rhs) => {
                rhs.pretty(f, level + 1)?;
                writeln!(f, "{}── /", "   │".repeat(level))?;
                lhs.pretty(f, level + 1)
            }
            Expr::ExactDiv(lhs, rhs) => {
                rhs.pretty(f, level + 1)?;
                writeln!(f, "{}── //", "   │".repeat(level))?;
                lhs.pretty(f, level + 1)
            }
        }
    }

    fn eval(&self) -> Num {
        match self {
            Expr::Num(num) => num.clone(),
            Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expr::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
            Expr::ExactDiv(lhs, rhs) => lhs.eval().exact_div(rhs.eval()),
            // Expr::Abs(ohs) => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Num {
    Integer(i128),
    Float(f64),
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer + rhs_integer),
                Num::Float(rhs_double) => Num::Float(lhs_integer as f64 + rhs_double),
            },
            Num::Float(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_double + rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_double + rhs_double),
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
                Num::Float(rhs_double) => Num::Float(lhs_integer as f64 - rhs_double),
            },
            Num::Float(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_double - rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_double - rhs_double),
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
                Num::Float(rhs_double) => Num::Float(lhs_integer as f64 * rhs_double),
            },
            Num::Float(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_double * rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_double * rhs_double),
            },
        }
    }
}

impl Div for Num {
    type Output = Num;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_integer as f64 / rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_integer as f64 / rhs_double),
            },
            Num::Float(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_double / rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_double / rhs_double),
            },
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Integer(value) => write!(f, "{value}"),
            Num::Float(value) => write!(f, "{value}"),
        }
    }
}

impl Num {
    fn exact_div(self, rhs: Num) -> Num {
        match self {
            Num::Integer(lhs_integer) => match rhs {
                Num::Integer(rhs_integer) => Num::Integer(lhs_integer / rhs_integer),
                Num::Float(rhs_double) => Num::Float(lhs_integer as f64 / rhs_double),
            },
            Num::Float(lhs_double) => match rhs {
                Num::Integer(rhs_integer) => Num::Float(lhs_double / rhs_integer as f64),
                Num::Float(rhs_double) => Num::Float(lhs_double / rhs_double),
            },
        }
    }
}

type VResult<I, O> = IResult<I, O, VerboseError<I>>;

fn parse_float(input: &str) -> VResult<&str, &str> {
    recognize(tuple((
        opt(alt((char('+'), char('-')))),
        alt((
            map(tuple((digit1, pair(char('.'), opt(digit1)))), |_| ()),
            map(tuple((char('.'), digit1)), |_| ()),
        )),
        opt(tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            cut(digit1),
        ))),
    )))(input)
}

fn parse_num(input: &str) -> VResult<&str, Expr> {
    // FIXME: handle +-
    delimited(
        space0,
        alt((
            map_res(context("float", parse_float), |digit_str: &str| {
                f64::from_str(digit_str).map(|value: f64| Expr::Num(Num::Float(value)))
            }),
            map_res(context("integer", digit1), |digit_str: &str| {
                i128::from_str(digit_str).map(|value: i128| Expr::Num(Num::Integer(value)))
            }),
        )),
        space0,
    )(input)
}

fn parse_parens(input: &str) -> VResult<&str, Expr> {
    delimited(
        context("left parentheses", delimited(space0, char('('), space0)),
        parse_expr,
        context("right parentheses", delimited(space0, char(')'), space0)),
    )(input)
}

fn parse_factor(input: &str) -> VResult<&str, Expr> {
    alt((parse_num, parse_parens))(input)
}

fn parse_term(input: &str) -> VResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    let mut parse = fold_many0(
        pair(alt((tag("*"), tag("//"), tag("/"))), parse_factor),
        || init.clone(),
        |acc_expr: Expr, (op, expr): (&str, Expr)| match op {
            "*" => Expr::Mul(Box::new(acc_expr), Box::new(expr)),
            "/" => Expr::Div(Box::new(acc_expr), Box::new(expr)),
            "//" => Expr::ExactDiv(Box::new(acc_expr), Box::new(expr)),
            other => {
                panic!("unexpected operator: {}", other)
            }
        },
    );
    parse(input)
}

fn parse_expr(input: &str) -> VResult<&str, Expr> {
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
        let input = "3.1 + 5 * (2 - 8) // 4";
        match parse_expr(input) {
            Ok((remaining, expr)) => {
                println!("{}", remaining);
                println!("{:?}", expr);
                println!("{:?}", expr.eval())
            }
            Err(err) => println!("{:?}", err),
        }
    }
}
