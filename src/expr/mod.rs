mod tokenizer;
mod parser;
pub mod eval;

use std::io::Result;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Func0Call(String),
    Func1Call(String, Box<Expr>),
    Func2Call(String, Box<[Expr; 2]>),
    Add(Box<[Expr; 2]>),
    Sub(Box<[Expr; 2]>),
    Mul(Box<[Expr; 2]>),
    Div(Box<[Expr; 2]>),
    Pow(Box<[Expr; 2]>),
    Minus(Box<Expr>),
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self> {
        let mut parser = parser::Parser::new(input);
        parser.parse()
    }
}
