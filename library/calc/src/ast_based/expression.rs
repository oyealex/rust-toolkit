use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub(crate) enum Expr {
    Num(Num),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    ExactDiv(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        crate::ast_based::pretty::pretty(self, f, 0)
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
            Expr::ExactDiv(lhs, rhs) => lhs.eval().exact_div(rhs.eval()),
            Expr::Mod(lhs, rhs) => lhs.eval().exact_div(rhs.eval()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Num {
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
    fn modulus(self, rhs: Num) -> Num {
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
