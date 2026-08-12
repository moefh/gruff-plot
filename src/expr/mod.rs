mod tokenizer;
mod parser;
mod analyzer;

use std::collections::HashMap;
use std::io::Result;

mod funcs {
    pub fn log(x: f64) -> f64 { x.ln() }
    pub fn exp(x: f64) -> f64 { x.exp() }
    pub fn sin(x: f64) -> f64 { x.sin() }
    pub fn cos(x: f64) -> f64 { x.cos() }
    pub fn sqrt(x: f64) -> f64 { x.sqrt() }
    pub fn abs(x: f64) -> f64 { x.abs() }

    pub fn min(x: f64, y: f64) -> f64 { x.min(y) }
    pub fn max(x: f64, y: f64) -> f64 { x.max(y) }
    pub fn module(x: f64, y: f64) -> f64 { x % y }
}

pub type Func0 = fn() -> f64;
pub type Func1 = fn(f64) -> f64;
pub type Func2 = fn(f64, f64) -> f64;

#[derive(Debug)]
struct EnvSymbols {
    vars: HashMap<String, usize>,
    funcs0: HashMap<String, usize>,
    funcs1: HashMap<String, usize>,
    funcs2: HashMap<String, usize>,
}

impl EnvSymbols {
    fn new() -> Self {
        EnvSymbols {
            vars: HashMap::new(),
            funcs0: HashMap::new(),
            funcs1: HashMap::new(),
            funcs2: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct EnvValues {
    vars: Vec<f64>,
    funcs0: Vec<Func0>,
    funcs1: Vec<Func1>,
    funcs2: Vec<Func2>,
}

pub struct Environment {
    symbols: EnvSymbols,
    values: EnvValues,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbols: EnvSymbols::new(),
            values: EnvValues {
                vars: Vec::new(),
                funcs0: Vec::new(),
                funcs1: Vec::new(),
                funcs2: Vec::new(),
            },
        }
    }

    pub fn with_math_functions(mut self) -> Self {
        self.load_math_functions();
        self
    }

    pub fn with_math_constants(mut self) -> Self {
        self.load_math_constants();
        self
    }

    pub fn with_var(mut self, var: impl Into<String>, value: f64) -> Self {
        self.add_var(var, value);
        self
    }

    pub fn load_math_constants(&mut self) {
        self.add_var("e", std::f64::consts::E);
        self.add_var("pi", std::f64::consts::PI);
    }

    pub fn load_math_functions(&mut self) {
        self.add_func1("log", funcs::log);
        self.add_func1("exp", funcs::exp);
        self.add_func1("sqrt", funcs::sqrt);
        self.add_func1("sin", funcs::sin);
        self.add_func1("cos", funcs::cos);
        self.add_func1("abs", funcs::abs);

        self.add_func2("min", funcs::min);
        self.add_func2("max", funcs::max);
        self.add_func2("mod", funcs::module);
    }

    pub fn add_func0(&mut self, name: impl Into<String>, value: Func0) -> bool {
        let name = name.into();
        if ! self.symbols.funcs0.contains_key(&name) {
            let index = self.values.funcs0.len();
            self.values.funcs0.push(value);
            self.symbols.funcs0.insert(name, index);
            true
        } else {
            self.set_func0(&name, value);
            false
        }
    }

    pub fn add_func1(&mut self, name: impl Into<String>, value: Func1) -> bool {
        let name = name.into();
        if ! self.symbols.funcs1.contains_key(&name) {
            let index = self.values.funcs1.len();
            self.values.funcs1.push(value);
            self.symbols.funcs1.insert(name, index);
            true
        } else {
            self.set_func1(&name, value);
            false
        }
    }

    pub fn add_func2(&mut self, name: impl Into<String>, value: Func2) -> bool {
        let name = name.into();
        if ! self.symbols.funcs2.contains_key(&name) {
            let index = self.values.funcs2.len();
            self.values.funcs2.push(value);
            self.symbols.funcs2.insert(name, index);
            true
        } else {
            self.set_func2(&name, value);
            false
        }
    }

    pub fn add_var(&mut self, name: impl Into<String>, value: f64) -> bool {
        let name = name.into();
        if ! self.symbols.vars.contains_key(&name) {
            let index = self.values.vars.len();
            self.values.vars.push(value);
            self.symbols.vars.insert(name, index);
            true
        } else {
            self.set_var(&name, value);
            false
        }
    }

    pub fn set_func0(&mut self, name: impl AsRef<str>, value: Func0) -> bool {
        if let Some(index) = self.symbols.funcs0.get(name.as_ref()) {
            self.values.funcs0[*index] = value;
            true
        } else {
            false
        }
    }

    pub fn set_func1(&mut self, name: impl AsRef<str>, value: Func1) -> bool {
        if let Some(index) = self.symbols.funcs1.get(name.as_ref()) {
            self.values.funcs1[*index] = value;
            true
        } else {
            false
        }
    }

    pub fn set_func2(&mut self, name: impl AsRef<str>, value: Func2) -> bool {
        if let Some(index) = self.symbols.funcs2.get(name.as_ref()) {
            self.values.funcs2[*index] = value;
            true
        } else {
            false
        }
    }

    pub fn set_var(&mut self, var: impl AsRef<str>, val: f64) -> bool {
        if let Some(index) = self.symbols.vars.get(var.as_ref()) {
            self.values.vars[*index] = val;
            true
        } else {
            false
        }
    }

    pub fn get_func0_index(&self, name: impl AsRef<str>) -> Option<usize> {
        self.symbols.funcs0.get(name.as_ref()).copied()
    }

    pub fn get_func1_index(&self, name: impl AsRef<str>) -> Option<usize> {
        self.symbols.funcs1.get(name.as_ref()).copied()
    }

    pub fn get_func2_index(&self, name: impl AsRef<str>) -> Option<usize> {
        self.symbols.funcs2.get(name.as_ref()).copied()
    }

    pub fn get_var_index(&self, name: impl AsRef<str>) -> Option<usize> {
        self.symbols.vars.get(name.as_ref()).copied()
    }
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(usize),
    Func0Call(usize),
    Func1Call(usize, Box<Expr>),
    Func2Call(usize, Box<[Expr; 2]>),
    Add(Box<[Expr; 2]>),
    Sub(Box<[Expr; 2]>),
    Mul(Box<[Expr; 2]>),
    Div(Box<[Expr; 2]>),
    Pow(Box<[Expr; 2]>),
    Minus(Box<Expr>),
}

impl Expr {
    pub const ZERO: Self = Expr::Number(0.0);

    pub fn parse(input: &str, env: &Environment) -> Result<Self> {
        let mut parser = parser::Parser::new(input);
        let ast = parser.parse()?;
        let expr = analyzer::analyze(&ast, &env.symbols)?;
        Ok(expr)
    }

    pub fn evaluate(&self, env: &Environment) -> f64 {
        self.eval(&env.values)
    }

    fn eval(&self, vals: &EnvValues) -> f64 {
        match self {
            Expr::Number(num) => { *num }
            Expr::Variable(var_index) => { vals.vars[*var_index] }

            Expr::Minus(arg) => { -arg.eval(vals) }

            Expr::Add(args) => { args[0].eval(vals) + args[1].eval(vals) }
            Expr::Sub(args) => { args[0].eval(vals) - args[1].eval(vals) }
            Expr::Mul(args) => { args[0].eval(vals) * args[1].eval(vals) }
            Expr::Div(args) => { args[0].eval(vals) / args[1].eval(vals) }

            Expr::Pow(args) => { args[0].eval(vals).powf(args[1].eval(vals)) }

            Expr::Func0Call(func_index) => {
                vals.funcs0.get(*func_index).map(|f| f()).unwrap_or(0.0)
            }

            Expr::Func1Call(func_index, arg) => {
                let arg = arg.eval(vals);
                vals.funcs1.get(*func_index).map(|f| f(arg)).unwrap_or(0.0)
            }

            Expr::Func2Call(func_index, args) => {
                let arg0 = args[0].eval(vals);
                let arg1 = args[1].eval(vals);
                vals.funcs2.get(*func_index).map(|f| f(arg0, arg1)).unwrap_or(0.0)
            }
        }
    }
}
