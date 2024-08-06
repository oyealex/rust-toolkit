use std::str::FromStr;

use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    IResult,
    multi::fold_many0,
    sequence::{delimited, pair},
};

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn parse_num(input: &str) -> IResult<&str, Expr> {
    map_res(delimited(space0, digit1, space0), |digit_str: &str| {
        i32::from_str(digit_str).map(Expr::Num)
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

    fold_many0(
        pair(alt((char('*'), char('/'))), parse_factor),
        || Expr::Num(0),
        |acc, (op, expr): (char, Expr)| {
            if op == '*' {
                Expr::Mul(Box::new(acc), Box::new(expr))
            } else {
                Expr::Div(Box::new(acc), Box::new(expr))
            }
        },
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;

    fold_many0(
        pair(alt((char('+'), char('-'))), parse_term),
        || Expr::Num(0),
        |acc, (op, expr): (char, Expr)| {
            if op == '+' {
                Expr::Add(Box::new(acc), Box::new(expr))
            } else {
                Expr::Sub(Box::new(acc), Box::new(expr))
            }
        },
    )(input)
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Sub(lhs, rhs) => eval(lhs) - eval(rhs),
        Expr::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
        Expr::Div(lhs, rhs) => eval(lhs) / eval(rhs),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn test() {
        let input = "3 + 5 * ( 10 - 4 ) / 2";
        match parse_expr(input) {
            Ok((_, expr)) => {
                println!("Parsed expression: {:?}", expr);
                println!("Evaluated result: {}", eval(&expr));
            }
            Err(err) => {
                println!("Failed to parse expression: {:?}", err);
            }
        }
    }
}
