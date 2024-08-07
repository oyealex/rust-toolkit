use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt, recognize};
use nom::multi::fold_many0;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum ErrKind {
    UnExpectedOperator,
    UnExpectedNumber,
    MoreParamRequired,
    InvalidExpression,
}

#[derive(Debug, PartialEq)]
enum ExpressionItem {
    Number(Number),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
enum Number {
    Integer(i128),
    // Double(f64),
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::Integer(value)
    }
}

// impl From<f64> for Number {
//     fn from(value: f64) -> Self {
//         Number::Double(value)
//     }
// }

#[derive(Debug)]
enum Operator {
    Eval,
    Add,
    Subtract,
    Multiply,
    Divide,
    // LeftParenthesis,
    // RightParenthesis,
}

impl Operator {
    fn calculate(&self, numbers: &mut Vec<Number>) -> Result<Number, ErrKind> {
        // FIXME: duplicated code need optimize
        if !self.is_param_enough(numbers.len()) {
            return Err(ErrKind::MoreParamRequired);
        }
        match self {
            Operator::Add => {
                let right = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                let left = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                Ok(Number::from(left + right))
            }
            Operator::Subtract => {
                let right = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                let left = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                Ok(Number::from(left - right))
            }
            Operator::Multiply => {
                let right = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                let left = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                Ok(Number::from(left * right))
            }
            Operator::Divide => {
                let right = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                let left = match numbers.pop().unwrap() {
                    Number::Integer(value) => value,
                };
                Ok(Number::from(left / right))
            }
            Operator::Eval => Ok(numbers.pop().unwrap()),
        }
    }

    fn priority(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Eval => 0,
        }
    }

    fn excepted_param_count(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => 2,
            Operator::Eval => 1,
        }
    }

    fn is_param_enough(&self, len: usize) -> bool {
        self.excepted_param_count() as usize <= len
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
    can_accept_number: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            numbers: Vec::new(),
            operators: Vec::new(),
            can_accept_number: true,
        }
    }
}

impl Calculator {
    fn append_number(&mut self, number: Number) -> Result<(), ErrKind> {
        if self.can_accept_number {
            self.numbers.push(number);
            self.can_accept_number = false;
            Ok(())
        } else {
            Err(ErrKind::UnExpectedNumber)
        }
    }

    /// 8 + 2 - 4 * 5 / 2
    /// numbers:    8 2
    /// operators:  +
    fn append_operator(&mut self, operator: Operator) -> Result<(), ErrKind> {
        if self.can_accept_number {
            return Err(ErrKind::UnExpectedOperator);
        }
        loop {
            match self.operators.last() {
                None => {
                    // no operator
                    return if self.numbers.is_empty() {
                        // no number, operator not allowed
                        Err(ErrKind::UnExpectedOperator)
                    } else {
                        break;
                    };
                }
                Some(top_operator) => {
                    // the new operator's priority is lower,
                    // calculate the top operator's result first
                    if &operator <= top_operator {
                        let top_operator = self.operators.pop().unwrap();
                        let result = top_operator.calculate(&mut self.numbers)?;
                        self.numbers.push(result);
                    } else {
                        break;
                    }
                }
            }
        }
        self.operators.push(operator);
        self.can_accept_number = true;
        Ok(())
    }

    fn get_final_result(&mut self) -> Result<&Number, ErrKind> {
        // push the lowest operator to force eval all remaining operator
        self.append_operator(Operator::Eval)?;
        // check the result
        if self.numbers.len() != 1
            || self.operators.len() != 1
            || self.operators.last() != Some(&Operator::Eval)
        {
            Err(ErrKind::InvalidExpression)
        } else {
            Ok(self.numbers.first().unwrap())
        }
    }
}

fn num(input: &str) -> IResult<&str, ExpressionItem> {
    map(
        // recognize(tuple((opt(alt((char('+'), char('-')))), digit1))),
        recognize(digit1),
        |s: &str| ExpressionItem::Number(Number::from(s.parse::<i128>().unwrap())),
    )
    .parse(input)
}

fn operator(input: &str) -> IResult<&str, ExpressionItem> {
    alt((
        map(char('+'), |_| ExpressionItem::Operator(Operator::Add)),
        map(char('-'), |_| ExpressionItem::Operator(Operator::Subtract)),
        map(char('*'), |_| ExpressionItem::Operator(Operator::Multiply)),
        map(char('/'), |_| ExpressionItem::Operator(Operator::Divide)),
    ))
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Calculator> {
    fold_many0(
        alt((num, operator)),
        Calculator::default,
        |mut calc, item| {
            println!("item: {item:?}");
            match item {
                ExpressionItem::Number(number) => calc.append_number(number).unwrap(),
                ExpressionItem::Operator(operator) => calc.append_operator(operator).unwrap(),
            }
            calc
        },
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::stack_based::{num, operator, parse};
    use crate::stack_based::{Calculator, ErrKind, ExpressionItem, Number, Operator};

    #[test]
    fn test_num_parse() {
        assert_eq!(
            Ok(("", ExpressionItem::Number(Number::from(123)))),
            num("123")
        );
        assert_eq!(
            Ok(("", ExpressionItem::Number(Number::from(123)))),
            num("+123")
        );
        assert_eq!(
            Ok(("", ExpressionItem::Number(Number::from(-123)))),
            num("-123")
        );
    }

    #[test]
    fn test_operator_parse() {
        assert_eq!(
            Ok(("", ExpressionItem::Operator(Operator::Add))),
            operator("+")
        );
        assert_eq!(
            Ok(("", ExpressionItem::Operator(Operator::Subtract))),
            operator("-")
        );
        assert_eq!(
            Ok(("", ExpressionItem::Operator(Operator::Multiply))),
            operator("*")
        );
        assert_eq!(
            Ok(("", ExpressionItem::Operator(Operator::Divide))),
            operator("/")
        );
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

    #[test]
    fn test_calculator() {
        // 8 + 2 - 4 * 5 / 2
        let mut calc = Calculator::default();
        assert_eq!(Ok(()), calc.append_number(Number::from(8_i128)));
        assert_eq!(
            Err(ErrKind::UnExpectedNumber),
            calc.append_number(Number::from(8_i128))
        );
        assert_eq!(Ok(()), calc.append_operator(Operator::Add));
        assert_eq!(
            Err(ErrKind::UnExpectedOperator),
            calc.append_operator(Operator::Add)
        );
        assert_eq!(Ok(()), calc.append_number(Number::from(2_i128)));
        assert_eq!(Ok(()), calc.append_operator(Operator::Subtract));
        assert_eq!(Ok(()), calc.append_number(Number::from(4_i128)));
        assert_eq!(Ok(()), calc.append_operator(Operator::Multiply));
        assert_eq!(Ok(()), calc.append_number(Number::from(5_i128)));
        assert_eq!(Ok(()), calc.append_operator(Operator::Divide));
        assert_eq!(Ok(()), calc.append_number(Number::from(2_i128)));
        assert_eq!(Ok(&Number::from(0_i128)), calc.get_final_result());
    }

    #[test]
    fn test_parse() {
        let mut result = parse("8+2-4*5/2");
        println!("{result:?}");
        println!("{:?}", result.unwrap().1.get_final_result());
    }
}
