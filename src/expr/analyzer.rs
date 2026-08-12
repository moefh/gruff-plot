use std::collections::HashMap;
use std::io::{Result, Error};

use super::parser::ExprAst;
use super::{
    Expr,
    EnvSymbols,
};

fn get_var(sym: &str, map: &HashMap<String, usize>) -> Result<usize> {
    map.get(sym).ok_or_else(|| {
        Error::other(format!("unknown variable: '{}'", sym))
    }).copied()
}

fn get_func(sym: &str, map: &HashMap<String, usize>) -> Result<usize> {
    map.get(sym).ok_or_else(|| {
        Error::other(format!("unknown function: '{}'", sym))
    }).copied()
}

pub fn analyze(ast: &ExprAst, symbols: &EnvSymbols) -> Result<Expr> {
    match ast {
        ExprAst::Number(n) => {
            Ok(Expr::Number(*n))
        }

        ExprAst::Variable(sym) => {
            let var = get_var(sym, &symbols.vars)?;
            Ok(Expr::Variable(var))
        }
        ExprAst::Func0Call(sym) => {
            let func = get_func(sym, &symbols.funcs0)?;
            Ok(Expr::Func0Call(func))
        }
        ExprAst::Func1Call(sym, arg) => {
            let func = get_func(sym, &symbols.funcs1)?;
            let arg = analyze(arg, symbols)?;
            Ok(Expr::Func1Call(func, Box::new(arg)))
        }
        ExprAst::Func2Call(sym, args) => {
            let func = get_func(sym, &symbols.funcs2)?;
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Func2Call(func, Box::new([arg0, arg1])))
        }

        ExprAst::Add(args) => {
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Add(Box::new([arg0, arg1])))
        }
        ExprAst::Sub(args) => {
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Sub(Box::new([arg0, arg1])))
        }
        ExprAst::Mul(args) => {
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Mul(Box::new([arg0, arg1])))
        }
        ExprAst::Div(args) => {
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Div(Box::new([arg0, arg1])))
        }
        ExprAst::Pow(args) => {
            let arg0 = analyze(&args[0], symbols)?;
            let arg1 = analyze(&args[1], symbols)?;
            Ok(Expr::Pow(Box::new([arg0, arg1])))
        }
        ExprAst::Minus(arg) => {
            let arg = analyze(arg, symbols)?;
            Ok(Expr::Minus(Box::new(arg)))
        }
    }
}
