use std::collections::HashMap;

use super::Expr;

pub type Func0 = fn() -> f64;
pub type Func1 = fn(f64) -> f64;
pub type Func2 = fn(f64, f64) -> f64;

mod funcs {
    pub fn log(arg: f64) -> f64 { arg.ln() }
    pub fn exp(arg: f64) -> f64 { arg.exp() }
    pub fn sin(arg: f64) -> f64 { arg.sin() }
    pub fn cos(arg: f64) -> f64 { arg.cos() }
    pub fn sqrt(arg: f64) -> f64 { arg.sqrt() }

    pub fn module(x: f64, y: f64) -> f64 { x % y }
}

pub struct ExprEvaluator {
    pub vars: HashMap<String, f64>,
    pub funcs0: HashMap<String, Func0>,
    pub funcs1: HashMap<String, Func1>,
    pub funcs2: HashMap<String, Func2>,
}

impl ExprEvaluator {
    pub fn new() -> Self {
        ExprEvaluator {
            funcs0: HashMap::new(),
            funcs1: HashMap::new(),
            funcs2: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn with_math_funcs(mut self) -> Self {
        self.load_math_funcs();
        self
    }

    pub fn with_math_consts(mut self) -> Self {
        self.load_math_constants();
        self
    }

    pub fn load_math_funcs(&mut self) {
        self.funcs1.insert(String::from("exp"), funcs::exp);
        self.funcs1.insert(String::from("log"), funcs::log);
        self.funcs1.insert(String::from("sin"), funcs::sin);
        self.funcs1.insert(String::from("cos"), funcs::cos);
        self.funcs1.insert(String::from("sqrt"), funcs::sqrt);

        self.funcs2.insert(String::from("mod"), funcs::module);
    }

    pub fn load_math_constants(&mut self) {
        self.vars.insert(String::from("e"), std::f64::consts::E);
        self.vars.insert(String::from("pi"), std::f64::consts::PI);
    }

    pub fn set_var(&mut self, var: &str, val: f64) {
        self.vars.insert(var.to_owned(), val);
    }

    pub fn eval(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(num) => { *num }
            Expr::Variable(name) => { self.vars.get(name).copied().unwrap_or(0.0) }

            Expr::Minus(arg) => { -self.eval(arg) }

            Expr::Add(args) => { self.eval(&args[0]) + self.eval(&args[1]) }
            Expr::Sub(args) => { self.eval(&args[0]) - self.eval(&args[1]) }
            Expr::Mul(args) => { self.eval(&args[0]) * self.eval(&args[1]) }
            Expr::Div(args) => { self.eval(&args[0]) / self.eval(&args[1]) }
            Expr::Pow(args) => { self.eval(&args[0]).powf(self.eval(&args[1])) }

            Expr::Func0Call(func) => {
                self.funcs0.get(func).map(|f| f()).unwrap_or(0.0)
            }

            Expr::Func1Call(func, arg) => {
                let arg_val = self.eval(arg);
                self.funcs1.get(func).map(|f| f(arg_val)).unwrap_or(0.0)
            }

            Expr::Func2Call(func, args) => {
                let arg0_val = self.eval(&args[0]);
                let arg1_val = self.eval(&args[1]);
                self.funcs2.get(func).map(|f| f(arg0_val, arg1_val)).unwrap_or(0.0)
            }
        }
    }
}
