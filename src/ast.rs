/* Abstract syntax tree for Scheme. */

use std::fmt;

pub enum Expr {
    IntExpr(i64)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::IntExpr(v) => {
                write!(f, "{}", v)
            }
        }
    }
}
