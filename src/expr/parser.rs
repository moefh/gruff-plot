use std::io::{Result, Error};

#[derive(Copy, Clone, Debug)]
enum Operator {
    UnaryMinus,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OperatorAssoc {
    Left,
    Right,
    Prefix,
}

#[derive(Copy, Clone, Debug)]
struct OperatorDef {
    ch: Option<char>,
    op: Operator,
    assoc: OperatorAssoc,
    arity: u8,
    prec: i8,
}

impl OperatorDef {
    fn new_expr(&self, opns: &mut Vec<Expr>) -> Option<Expr> {
        match self.op {
            Operator::UnaryMinus => { Some(Expr::Minus(Box::new(opns.pop()?))) }
            Operator::Add => { let r = opns.pop()?; let l = opns.pop()?; Some(Expr::Add(Box::new([l, r]))) }
            Operator::Sub => { let r = opns.pop()?; let l = opns.pop()?; Some(Expr::Sub(Box::new([l, r]))) }
            Operator::Mul => { let r = opns.pop()?; let l = opns.pop()?; Some(Expr::Mul(Box::new([l, r]))) }
            Operator::Div => { let r = opns.pop()?; let l = opns.pop()?; Some(Expr::Div(Box::new([l, r]))) }
            Operator::Pow => { let r = opns.pop()?; let l = opns.pop()?; Some(Expr::Pow(Box::new([l, r]))) }
        }
    }
}

const PREC_FUNC_CALL: i8 = i8::MAX;

const OPERATORS: &[OperatorDef] = &[
    OperatorDef { arity: 2, ch: Some('+'), op: Operator::Add,        assoc: OperatorAssoc::Left,   prec: 10 },
    OperatorDef { arity: 2, ch: Some('-'), op: Operator::Sub,        assoc: OperatorAssoc::Left,   prec: 10 },
    OperatorDef { arity: 2, ch: Some('*'), op: Operator::Mul,        assoc: OperatorAssoc::Left,   prec: 20 },
    OperatorDef { arity: 2, ch: Some('/'), op: Operator::Div,        assoc: OperatorAssoc::Left,   prec: 20 },
    OperatorDef { arity: 1, ch: Some('-'), op: Operator::UnaryMinus, assoc: OperatorAssoc::Prefix, prec: 30 },
    OperatorDef { arity: 2, ch: Some('^'), op: Operator::Pow,        assoc: OperatorAssoc::Right,  prec: 40 },
];

fn is_operator(t: &Token) -> bool {
    if let Some(ch) = t.get_punct() {
        OPERATORS.iter().find(|op| Some(ch) == op.ch).is_some()
    } else {
        false
    }
}

fn get_operator(arity: u8, t: &Token) -> Option<&OperatorDef> {
    t.get_punct().and_then(|ch| OPERATORS.iter().find(|op| op.arity == arity && Some(ch) == op.ch))
}

use super::Expr;
use super::tokenizer::{
    Token,
    TokenPosition,
    Tokenizer,
};

pub struct Parser<'a> {
    tok: Tokenizer<'a>,
    unget_data: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            tok: Tokenizer::new(input),
            unget_data: Vec::new(),
        }
    }

    fn read(&mut self) -> Result<Token> {
        if let Some(t) = self.unget_data.pop() {
            return Ok(t);
        }
        self.tok.read()
    }

    fn unread(&mut self, t: Token) {
        self.unget_data.push(t);
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>> {
        // check empty list
        let t = self.read()?;
        if t.is_punct(')') {
            return Ok(Vec::new());
        }
        self.unread(t);

        // not empty; read expressions
        let mut list = Vec::new();
        loop {
            list.push(self.parse_expr(false, &[',', ')'])?);
            let t = self.read()?;
            if t.is_punct(')') {
                break;
            }
            if ! t.is_punct(',') {
                return Err(Error::other(format!("expected ',' or ')', found '{}'", t)));
            }
        }
        Ok(list)
    }

    fn resolve_expr_stack(
        &mut self,
        _pos: TokenPosition,
        opns: &mut Vec<Expr>,
        oprs: &mut Vec<(OperatorDef, TokenPosition)>,
        stop_prec: i8,
    ) -> Result<()> {
        while ! oprs.is_empty() {
            if let Some((op, pos)) = oprs.pop() {
                let op_prec = if op.assoc == OperatorAssoc::Right { op.prec-1 } else { op.prec };
                if op_prec < stop_prec {
                    oprs.push((op, pos));
                    break;
                }
                let expr = op.new_expr(opns).ok_or(Error::other("syntax error"))?;
                opns.push(expr);
            }
        }
        Ok(())
    }

    fn parse_expr(&mut self, consume_stop: bool, stop_chars: &[char]) -> Result<Expr> {
        let mut expect_opn = true;
        let mut opns = Vec::new();
        let mut oprs = Vec::new();

        loop {
            let mut t = self.read()?;

            // ( ... )
            if t.is_punct('(') {
                let expr = if expect_opn {
                    expect_opn = false;
                    self.parse_expr(true, &[')'])?
                } else {
                    self.resolve_expr_stack(t.pos, &mut opns, &mut oprs, PREC_FUNC_CALL)?;
                    let func = opns.pop().ok_or(Error::other("syntax error (no function on stack)"))?;
                    let func_name = if let Expr::Variable(func_name) = func {
                        func_name
                    } else {
                        return Err(Error::other("function must be an identifier"));
                    };
                    let mut args = self.parse_expr_list()?;
                    let mut arg0 = args.pop();
                    let mut arg1 = args.pop();
                    if ! args.is_empty() {
                        return Err(Error::other("too many function arguments"));
                    }
                    if let Some(a0) = arg0.take() {
                        if let Some(a1) = arg1.take() {
                            Expr::Func2Call(func_name, Box::new([a1, a0]))
                        } else {
                            Expr::Func1Call(func_name, Box::new(a0))
                        }
                    } else {
                        Expr::Func0Call(func_name)
                    }
                };
                opns.push(expr);
                continue;
            }

            // number
            if let Some(n) = t.get_number() {
                if ! expect_opn {
                    return Err(Error::other(format!("expected '(' or operator, got {}'", t)));
                }
                opns.push(Expr::Number(n));
                expect_opn = false;
                continue;
            }

            // operator
            if is_operator(&t) {
                let op = if expect_opn {
                    get_operator(1, &t).ok_or(Error::other(format!("unexpected '{}'", t)))?
                } else {
                    let op = get_operator(2, &t).ok_or(
                        Error::other(format!("expected '(' or binary operator, got '{}'", t))
                    )?;
                    self.resolve_expr_stack(t.pos, &mut opns, &mut oprs, op.prec)?;
                    expect_opn = true;
                    op
                };
                oprs.push((*op, t.pos));
                continue;
            }

            // identifier
            if let Some(ident) = t.drain_ident() {
                if ! expect_opn {
                    return Err(Error::other(format!("expected '(' or operator, found '{}'", t)));
                }
                opns.push(Expr::Variable(ident));
                expect_opn = false;
                continue;
            }

            // stop char
            if let Some(ch) = t.get_punct() && stop_chars.contains(&ch) {
                self.resolve_expr_stack(t.pos, &mut opns, &mut oprs, i8::MIN)?;
                if opns.len() > 1 {
                    return Err(Error::other("syntax error (stack not empty)"));
                }
                if opns.is_empty() {
                    return Err(Error::other(format!("unexpected '{}'", t)));
                }
                if ! consume_stop {
                    self.unread(t);
                }
                return opns.pop().ok_or(Error::other("syntax error (empty stack)"));
            }

            // EOF
            if t.is_eof() {
                if ! stop_chars.is_empty() {
                    return Err(Error::other("unexpected end of expression"));
                }
                self.resolve_expr_stack(t.pos, &mut opns, &mut oprs, i8::MIN)?;
                if opns.len() > 1 {
                    return Err(Error::other("syntax error (stack not empty)"));
                }
                return opns.pop().ok_or(Error::other(format!("unexpected '{}'", t)));
            }

            return Err(Error::other(format!("unexpected: '{}'", t)));
        }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.parse_expr(true, &[])
    }
}
