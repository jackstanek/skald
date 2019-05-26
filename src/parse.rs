/* Hand-written recursive descent parser. */
extern crate regex;

use crate::ast::Expr;

enum Token {
    Integer(i64),
    Bool(bool),
    Char(char)
}

#[derive(Debug)]
pub enum ErrorCause {
    InvalidToken(String)
}

#[derive(Debug)]
pub struct ParseError {
    cause: ErrorCause,
    row: u16,
    col: u16
}

fn lex_input(input: &String) -> Result<Vec<Token>, ParseError> {
    Ok(vec![Token::Integer(420)])
}

fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<Expr>, ParseError> {
    Ok(vec![Expr::IntExpr(420)])
}

pub fn parse_input(input: &String) -> Result<Vec<Expr>, ParseError> {
    let mut lexed = lex_input(input);
    parse_tokens(&lexed.unwrap())
}
