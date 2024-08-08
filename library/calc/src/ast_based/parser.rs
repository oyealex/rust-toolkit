use crate::ast_based::VResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{cut, map, map_res, opt, recognize};
use nom::error::context;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, tuple};
use std::str::FromStr;

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

fn parse_num(input: &str) -> VResult<&str, crate::ast_based::expression::Expr> {
    // FIXME: handle +-
    delimited(
        space0,
        alt((
            map_res(context("float", parse_float), |digit_str: &str| {
                f64::from_str(digit_str).map(|value: f64| {
                    crate::ast_based::expression::Expr::Num(
                        crate::ast_based::expression::Num::Float(value),
                    )
                })
            }),
            map_res(context("integer", digit1), |digit_str: &str| {
                i128::from_str(digit_str).map(|value: i128| {
                    crate::ast_based::expression::Expr::Num(
                        crate::ast_based::expression::Num::Integer(value),
                    )
                })
            }),
        )),
        space0,
    )(input)
}

fn parse_parentheses(input: &str) -> VResult<&str, crate::ast_based::expression::Expr> {
    delimited(
        context("left parentheses", delimited(space0, char('('), space0)),
        parse_expr,
        context("right parentheses", delimited(space0, char(')'), space0)),
    )(input)
}

/// 从给定的输入字符串中解析最基础的表达式。
///
/// 主要包含：
/// - 普通数字
/// - 被括号包围的表达式
///
/// # Arguments
///
/// * `input`: 被解析的字符串。
///
/// returns: `Result<(&str, Expr), Err<VerboseError<&str>>>` 解析结果。
fn parse_factor(input: &str) -> VResult<&str, crate::ast_based::expression::Expr> {
    alt((parse_num, parse_parentheses))(input)
}

/// 从给定的输入字符串中解析高优先级的表达式。
///
/// 主要包含：
/// - `*`：乘法
/// - `/`：除法
/// - `//`：整除
/// - `%`：取余
///
/// # Arguments
///
/// * `input`: 被解析的字符串。
///
/// returns: `Result<(&str, Expr), Err<VerboseError<&str>>>` 解析结果。
fn parse_term(input: &str) -> VResult<&str, crate::ast_based::expression::Expr> {
    let (input, init) = parse_factor(input)?;
    let mut parse = fold_many0(
        pair(alt((tag("*"), tag("//"), tag("/"), tag("%"))), parse_factor),
        || init.clone(),
        |acc_expr: crate::ast_based::expression::Expr,
         (op, expr): (&str, crate::ast_based::expression::Expr)| match op {
            "*" => crate::ast_based::expression::Expr::Mul(Box::new(acc_expr), Box::new(expr)),
            "/" => crate::ast_based::expression::Expr::Div(Box::new(acc_expr), Box::new(expr)),
            "//" => {
                crate::ast_based::expression::Expr::ExactDiv(Box::new(acc_expr), Box::new(expr))
            }
            other => {
                panic!("unexpected operator: {}", other)
            }
        },
    );
    parse(input)
}

/// 从给定的输入字符串中解析低优先级的表达式。
///
/// 包含：
/// - `+`：加法
/// - `-`：减法
///
/// # Arguments
///
/// * `input`: 被解析的字符串。
///
/// returns: `Result<(&str, Expr), Err<VerboseError<&str>>>` 解析结果。
pub(crate) fn parse_expr(input: &str) -> VResult<&str, crate::ast_based::expression::Expr> {
    let (input, init) = parse_term(input)?;
    let mut parse = fold_many0(
        pair(alt((tag("+"), tag("-"))), parse_term),
        || init.clone(),
        |acc_expr: crate::ast_based::expression::Expr,
         (op, expr): (&str, crate::ast_based::expression::Expr)| match op {
            "+" => crate::ast_based::expression::Expr::Add(Box::new(acc_expr), Box::new(expr)),
            "-" => crate::ast_based::expression::Expr::Sub(Box::new(acc_expr), Box::new(expr)),
            other => {
                panic!("unexpected operator: {}", other)
            }
        },
    );
    parse(input)
}
