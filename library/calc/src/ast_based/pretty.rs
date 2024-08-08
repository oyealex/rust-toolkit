use crate::ast_based::expression::Expr;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
enum Arm {
    Left,
    Right,
    Root,
}

fn pretty0(expr: &Expr, f: &mut Formatter<'_>, arm: Arm, level: usize) -> std::fmt::Result {
    match expr {
        Expr::Num(ohs) => writeln!(f, "{}{}── {}", "   |".repeat(level - 1), get_arm(arm), ohs),
        Expr::Add(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── +", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
        Expr::Sub(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── -", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
        Expr::Mul(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── *", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
        Expr::Div(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── /", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
        Expr::ExactDiv(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── //", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
        Expr::Mod(lhs, rhs) => {
            pretty0(rhs, f, Arm::Right,level + 1)?;
            writeln!(f, "{}{}── //", "   |".repeat(level - 1), get_arm(arm))?;
            pretty0(lhs, f, Arm::Left,level + 1)
        }
    }

}

fn get_arm(arm: Arm) -> &'static str {
    match arm {
        Arm::Left => "   ╰",
        Arm::Right => "   ╭",
        Arm::Root => "",
    }
}

pub(crate) fn pretty(expr: &Expr, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
    pretty0(expr, f, Arm::Root, 0)
}

#[cfg(test)]
mod test {
    #[test]
    fn smoke() {
        use crate::ast_based::parser::parse_expr;
        let input = "3.1 + (1 - 2) * (2 - 8) // 4";
        // let input = "3+(1-2)";
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
