/* Abstract syntax tree for Scheme. */

use std::fmt;

pub enum Expr {
    IntExpr(i64),
    BoolExpr(bool),
    StringExpr(String),
    CharExpr(char)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::IntExpr(v) => {
                write!(f, "{}", v)
            },
            Expr::BoolExpr(b) => {
                write!(f, "{}", if *b { "#t" } else { "#f" })
            },
            Expr::StringExpr(s) => {
                write!(f, r#""{}""#, s)
            },
            Expr::CharExpr(c) => {
                write!(f, "#\\{}", c)
            }
        }
    }
}
