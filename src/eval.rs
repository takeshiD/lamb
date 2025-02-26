use crate::parser::{Atom, BuiltinOp, Expr};
use anyhow::Result;
use std::collections::HashMap;

// 環境（変数の名前と値を保持する）
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Expr>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    // 変数を定義する
    pub fn define(&mut self, name: String, value: Expr) {
        self.variables.insert(name, value);
    }
    
    // 変数の値を取得する
    pub fn lookup(&self, name: &str) -> Option<Expr> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
}

fn eval_apply(car: Box<Expr>, cdr: Vec<Expr>, env: &mut Environment) -> Result<Expr> {
    match *car {
        Expr::SelfEvaluation(atom) => match atom {
            Atom::Operater(op) => match op {
                BuiltinOp::Plus => {
                    let ret = cdr.into_iter().fold(Ok(0), |sum, e| match eval_expression(e, env) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(sum.unwrap() + n),
                            _ => Err(anyhow::anyhow!("{val:#?} is not a number.")),
                        },
                        Err(err) => Err(err),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret?)))
                }
                BuiltinOp::Minus => {
                    // 引数が1つ以上あるか確認
                    if cdr.is_empty() {
                        return Err(anyhow::anyhow!("- requires at least one argument"));
                    }
                    
                    let first = eval_expression(cdr[0].clone(), env)?;
                    let first_val = match first {
                        Expr::SelfEvaluation(Atom::Num(n)) => n,
                        _ => return Err(anyhow::anyhow!("{first:#?} is not a number.")),
                    };
                    
                    // 引数が1つだけなら単項マイナス、そうでなければ減算
                    if cdr.len() == 1 {
                        Ok(Expr::SelfEvaluation(Atom::Num(-first_val)))
                    } else {
                        let rest = cdr.into_iter().skip(1).fold(Ok(first_val), |acc, e| match eval_expression(e, env) {
                            Ok(val) => match val {
                                Expr::SelfEvaluation(Atom::Num(n)) => Ok(acc.unwrap() - n),
                                _ => Err(anyhow::anyhow!("{val:#?} is not a number.")),
                            },
                            Err(err) => Err(err),
                        });
                        Ok(Expr::SelfEvaluation(Atom::Num(rest?)))
                    }
                }
                BuiltinOp::Times => {
                    let ret = cdr.into_iter().fold(Ok(1), |product, e| match eval_expression(e, env) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(product.unwrap() * n),
                            _ => Err(anyhow::anyhow!("{val:#?} is not a number.")),
                        },
                        Err(err) => Err(err),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret?)))
                }
                BuiltinOp::Divide => {
                    if cdr.is_empty() {
                        return Err(anyhow::anyhow!("/ requires at least one argument"));
                    }
                    
                    let first = eval_expression(cdr[0].clone(), env)?;
                    let first_val = match first {
                        Expr::SelfEvaluation(Atom::Num(n)) => n,
                        _ => return Err(anyhow::anyhow!("{first:#?} is not a number.")),
                    };
                    
                    if cdr.len() == 1 {
                        // 単項の場合は逆数（1/n）を返す
                        if first_val == 0 {
                            return Err(anyhow::anyhow!("Division by zero"));
                        }
                        Ok(Expr::SelfEvaluation(Atom::Num(1 / first_val)))
                    } else {
                        let rest = cdr.into_iter().skip(1).fold(Ok(first_val), |acc, e| match eval_expression(e, env) {
                            Ok(val) => match val {
                                Expr::SelfEvaluation(Atom::Num(n)) => {
                                    if n == 0 {
                                        Err(anyhow::anyhow!("Division by zero"))
                                    } else {
                                        Ok(acc.unwrap() / n)
                                    }
                                },
                                _ => Err(anyhow::anyhow!("{val:#?} is not a number.")),
                            },
                            Err(err) => Err(err),
                        });
                        Ok(Expr::SelfEvaluation(Atom::Num(rest?)))
                    }
                }
            },
            Atom::Symbol(name) => {
                // シンボルの場合、環境から値を取得して適用する
                if let Some(func) = env.lookup(&name) {
                    match func {
                        Expr::SelfEvaluation(Atom::Operater(_)) => {
                            eval_apply(Box::new(func), cdr, env)
                        },
                        _ => Err(anyhow::anyhow!("{func:#?} is not applicable"))
                    }
                } else {
                    Err(anyhow::anyhow!("Undefined symbol: {name}"))
                }
            },
            _ => Err(anyhow::anyhow!("{atom:#?} is not an Operator or Symbol"))
        },
        _ => Err(anyhow::anyhow!("{car:#?} is not SelfEvaluation")),
    }
}

pub fn eval_expression(expr: Expr, env: &mut Environment) -> Result<Expr> {
    match expr {
        Expr::Application(car, cdr) => eval_apply(car, cdr, env),
        Expr::SelfEvaluation(atom) => match atom {
            Atom::Num(n) => Ok(Expr::SelfEvaluation(Atom::Num(n))),
            Atom::Boolean(b) => Ok(Expr::SelfEvaluation(Atom::Boolean(b))),
            Atom::Operater(op) => Ok(Expr::SelfEvaluation(Atom::Operater(op))),
            Atom::Symbol(name) => {
                // シンボルの場合、環境から値を取得
                if let Some(value) = env.lookup(&name) {
                    Ok(value)
                } else {
                    Err(anyhow::anyhow!("Undefined symbol: {name}"))
                }
            }
        },
        Expr::Define(name, expr) => {
            // 値を評価
            let value = eval_expression(*expr, env)?;
            // 環境に定義
            env.define(name, value.clone());
            // 定義された値を返す
            Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Atom, BuiltinOp, Expr};

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        let value = Expr::SelfEvaluation(Atom::Num(42));
        env.define("x".to_string(), value.clone());
        
        assert_eq!(env.lookup("x"), Some(value));
        assert_eq!(env.lookup("y"), None);
    }
    
    #[test]
    fn test_define() {
        let mut env = Environment::new();
        let define_expr = Expr::Define(
            "x".to_string(), 
            Box::new(Expr::SelfEvaluation(Atom::Num(42)))
        );
        
        let result = eval_expression(define_expr, &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::SelfEvaluation(Atom::Num(42)));
        
        // 定義された変数を参照
        let symbol_expr = Expr::SelfEvaluation(Atom::Symbol("x".to_string()));
        let result = eval_expression(symbol_expr, &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::SelfEvaluation(Atom::Num(42)));
    }
}
