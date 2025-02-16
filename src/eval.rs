use crate::parser::{Atom, BuiltinOp, Expr};
use anyhow::Result;

fn eval_apply(car: Box<Expr>, cdr: Vec<Expr>) -> Result<Expr> {
    match *car {
        Expr::SelfEvaluation(atom) => match atom {
            Atom::Operater(op) => match op {
                BuiltinOp::Plus => {
                    let ret = cdr.into_iter().fold(Ok(0), |sum, e| match eval_expression(e) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(sum.unwrap() + n),
                            _ => Err(anyhow::anyhow!("Failed")),
                        },
                        Err(_) => Err(anyhow::anyhow!("Failed")),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret.unwrap())))
                }
                BuiltinOp::Minus => {
                    let ret = cdr.into_iter().fold(Ok(0), |sum, e| match eval_expression(e) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(sum.unwrap() - n),
                            _ => Err(anyhow::anyhow!("Failed")),
                        },
                        Err(_) => Err(anyhow::anyhow!("Failed")),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret.unwrap())))
                }
                BuiltinOp::Times => {
                    let ret = cdr.into_iter().fold(Ok(1), |sum, e| match eval_expression(e) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(sum.unwrap() * n),
                            _ => Err(anyhow::anyhow!("Failed")),
                        },
                        Err(_) => Err(anyhow::anyhow!("Failed")),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret.unwrap())))
                }
                BuiltinOp::Divide => {
                    let ret = cdr.into_iter().fold(Ok(1), |sum, e| match eval_expression(e) {
                        Ok(val) => match val {
                            Expr::SelfEvaluation(Atom::Num(n)) => Ok(sum.unwrap() / n),
                            _ => Err(anyhow::anyhow!("Failed")),
                        },
                        Err(_) => Err(anyhow::anyhow!("Failed")),
                    });
                    Ok(Expr::SelfEvaluation(Atom::Num(ret.unwrap())))
                }
            },
            _ => Err(anyhow::anyhow!("Failed")),
        },
        _ => Err(anyhow::anyhow!("Failed")),
    }
}

pub fn eval_expression(expr: Expr) -> Result<Expr> {
    match expr {
        Expr::Application(car, cdr) => eval_apply(car, cdr),
        Expr::SelfEvaluation(atom) => match atom {
            Atom::Num(n) => Ok(Expr::SelfEvaluation(Atom::Num(n))),
            Atom::Boolean(b) => Ok(Expr::SelfEvaluation(Atom::Boolean(b))),
            _ => Err(anyhow::anyhow!("Failed")),
        },
    }
}
