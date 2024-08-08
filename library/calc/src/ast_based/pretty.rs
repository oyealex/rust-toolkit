use crate::ast_based::expression::Expr;
use std::fmt::Formatter;

pub(crate) fn pretty(expr: &Expr, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
    match expr {
        Expr::Num(ohs) => writeln!(f, "{}── {}", "   │".repeat(level), ohs),
        Expr::Add(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── +", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
        Expr::Sub(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── -", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
        Expr::Mul(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── *", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
        Expr::Div(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── /", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
        Expr::ExactDiv(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── //", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
        Expr::Mod(lhs, rhs) => {
            pretty(rhs, f, level + 1)?;
            writeln!(f, "{}── //", "   │".repeat(level))?;
            pretty(lhs, f, level + 1)
        }
    }
}
