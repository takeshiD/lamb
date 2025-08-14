use crate::parser::{Atom, BuiltinOp, Expr};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<String, Expr>,  // Symbol Table on Environment
    up: Option<Box<Environment>>, // Up scope environment
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
            up: None,
        }
    }
    pub fn add(&mut self, name: String, expr: Expr) {
        self.vars.insert(name, expr);
    }
    pub fn lookup(&self, name: &str) -> Option<Expr> {
        if let Some(expr) = self.vars.get(name) {
            Some(expr.clone())
        } else if let Some(up) = &self.up {
            up.lookup(name)
        } else {
            None
        }
    }
    pub fn push_env(up: Environment) -> Self {
        Environment {
            vars: HashMap::new(),
            up: Some(Box::new(up)),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

fn eval_apply(func: Expr, args: Vec<Expr>, env: &mut Environment) -> Result<Expr> {
    // 1) Evaluate the operator position
    let func_val = eval(func, env)?;
    // 2) Evaluate all arguments (applicative order)
    let mut evaled_args = Vec::with_capacity(args.len());
    for a in args {
        evaled_args.push(eval(a, env)?);
    }

    match func_val {
        // Builtin numeric operators
        Expr::SelfEvaluation(Atom::Operater(op)) => {
            // Extract i32 numbers from evaluated arguments
            let mut nums: Vec<i32> = Vec::with_capacity(evaled_args.len());
            for v in evaled_args {
                match v {
                    Expr::SelfEvaluation(Atom::Num(n)) => nums.push(n),
                    other => {
                        return Err(anyhow::anyhow!("expected number argument, got: {other:#?}"))
                    }
                }
            }

            let result = match op {
                BuiltinOp::Plus => nums.into_iter().sum(),
                BuiltinOp::Times => nums.into_iter().sum(),
                BuiltinOp::Minus => {
                    if nums.is_empty() {
                        return Err(anyhow::anyhow!("'-' requires at least 1 argument"));
                    }
                    let first = nums[0];
                    if nums.len() == 1 {
                        -first
                    } else {
                        first - nums[1..].iter().copied().sum::<i32>()
                    }
                }
                BuiltinOp::Divide => {
                    if nums.len() < 2 {
                        return Err(anyhow::anyhow!("'/' requires at least 2 arguments"));
                    }
                    let mut acc = nums[0];
                    for &n in &nums[1..] {
                        if n == 0 {
                            return Err(anyhow::anyhow!("division by zero"));
                        }
                        acc /= n;
                    }
                    acc
                }
            };

            Ok(Expr::SelfEvaluation(Atom::Num(result)))
        }
        // User-defined lambda (when implemented)
        Expr::Lambda(params, body) => {
            let mut local_env = Environment::push_env(env.clone());
            for (param, arg_val) in params.into_iter().zip(evaled_args.into_iter()) {
                if let Expr::Symbol(name) = param {
                    local_env.add(name, arg_val);
                }
            }
            eval(*body, &mut local_env)
        }
        other => Err(anyhow::anyhow!("'{other:#?}' is not applicable")),
    }
}

pub fn eval(expr: Expr, env: &mut Environment) -> Result<Expr> {
    match expr {
        Expr::Symbol(name) => {
            if let Some(e) = env.lookup(&name) {
                Ok(e)
            } else {
                Err(anyhow::anyhow!("Undefined symbol: '{name}'"))
            }
        }
        Expr::Procedure(func, args) => eval_apply(*func, args, env),
        Expr::SelfEvaluation(atom) => match atom {
            Atom::Num(n) => Ok(Expr::SelfEvaluation(Atom::Num(n))),
            Atom::Boolean(b) => Ok(Expr::SelfEvaluation(Atom::Boolean(b))),
            Atom::Operater(op) => Ok(Expr::SelfEvaluation(Atom::Operater(op))),
        },
        // Lambda evaluates to itself (a procedure value)
        Expr::Lambda(params, body) => Ok(Expr::Lambda(params, body)),
        Expr::Define(name, expr) => {
            let value = eval(*expr, env)?;
            env.add(name, value.clone());
            Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::parser::{Atom, BuiltinOp, Expr};

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        let value = Expr::SelfEvaluation(Atom::Num(42));
        env.add("x".to_string(), value.clone());

        assert_eq!(env.lookup("x"), Some(value));
        assert_eq!(env.lookup("y"), None);
    }

    #[test]
    fn test_eval_builtin_add_mul() {
        let mut env = Environment::new();
        // Register builtins like main.rs does
        env.add(
            "+".to_string(),
            Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus)),
        );
        env.add(
            "*".to_string(),
            Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Times)),
        );

        // (+ 1 2)
        let expr = Expr::Procedure(
            Box::new(Expr::Symbol("+".into())),
            vec![
                Expr::SelfEvaluation(Atom::Num(1)),
                Expr::SelfEvaluation(Atom::Num(2)),
            ],
        );
        let res = eval(expr, &mut env).unwrap();
        assert_eq!(res, Expr::SelfEvaluation(Atom::Num(3)));

        // (+ (* 2 3) 4) => 10
        let nested = Expr::Procedure(
            Box::new(Expr::Symbol("+".into())),
            vec![
                Expr::Procedure(
                    Box::new(Expr::Symbol("*".into())),
                    vec![
                        Expr::SelfEvaluation(Atom::Num(2)),
                        Expr::SelfEvaluation(Atom::Num(3)),
                    ],
                ),
                Expr::SelfEvaluation(Atom::Num(4)),
            ],
        );
        let res2 = eval(nested, &mut env).unwrap();
        assert_eq!(res2, Expr::SelfEvaluation(Atom::Num(10)));
    }

    #[test]
    fn test_eval_builtin_minus_divide() {
        let mut env = Environment::new();
        env.add(
            "-".to_string(),
            Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Minus)),
        );
        env.add(
            "/".to_string(),
            Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Divide)),
        );

        // (- 5 1 1) => 3
        let sub = Expr::Procedure(
            Box::new(Expr::Symbol("-".into())),
            vec![
                Expr::SelfEvaluation(Atom::Num(5)),
                Expr::SelfEvaluation(Atom::Num(1)),
                Expr::SelfEvaluation(Atom::Num(1)),
            ],
        );
        let res = eval(sub, &mut env).unwrap();
        assert_eq!(res, Expr::SelfEvaluation(Atom::Num(3)));

        // Unary minus: (- 5) => -5
        let unary = Expr::Procedure(
            Box::new(Expr::Symbol("-".into())),
            vec![Expr::SelfEvaluation(Atom::Num(5))],
        );
        let res2 = eval(unary, &mut env).unwrap();
        assert_eq!(res2, Expr::SelfEvaluation(Atom::Num(-5)));

        // (/ 8 2) => 4
        let div = Expr::Procedure(
            Box::new(Expr::Symbol("/".into())),
            vec![
                Expr::SelfEvaluation(Atom::Num(8)),
                Expr::SelfEvaluation(Atom::Num(2)),
            ],
        );
        let res3 = eval(div, &mut env).unwrap();
        assert_eq!(res3, Expr::SelfEvaluation(Atom::Num(4)));
    }

}
