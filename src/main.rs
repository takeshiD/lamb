use anyhow::Result;
use nom::error::convert_error;
use lamb::eval::{eval_expression, Environment};
use lamb::parser::{parse_expr, Atom, Expr};
use std::io::{self, Write};

static PROGRAM_NAME: &str = "lamb";
static HELP_MESSAGE: &str = "REPL Usage
    'help' '?'       show this help message
    'exit' 'quit'    finish lamb REPL
";

fn main() -> Result<()> {
    // 環境を作成
    let mut env = Environment::new();
    
    // 基本的な演算子を環境に追加
    env.define("+".to_string(), Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Plus)));
    env.define("-".to_string(), Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Minus)));
    env.define("*".to_string(), Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Times)));
    env.define("/".to_string(), Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Divide)));
    
    loop {
        let mut input = String::new();
        io::stdout()
            .write(format!("{PROGRAM_NAME}> ").as_bytes())
            .expect("Failed write to stdout");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed input");
        match input.trim() {
            "exit" | "quit" => {
                println!("Terminate {PROGRAM_NAME}");
                break;
            }
            "help" | "?" => {
                println!("{HELP_MESSAGE}");
                continue;
            }
            "env" => {
                // 環境内の変数を表示（デバッグ用）
                println!("Current environment: {:#?}", env);
                continue;
            }
            _ => (),
        }
        match parse_expr(&input) {
            Ok((_, expr)) => {
                match eval_expression(expr, &mut env) {
                    Ok(e) => match e {
                        Expr::SelfEvaluation(atom) => match atom {
                            Atom::Num(n) => println!("{n}"),
                            Atom::Boolean(b) => {
                                if b {
                                    println!("#t")
                                } else {
                                    println!("#f")
                                }
                            },
                            Atom::Symbol(s) => println!("{s}"),
                            Atom::Operater(_) => println!("<procedure>"),
                        },
                        _ => println!("Result: {:#?}", e),
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {:?}", e);
            }
        }
    }
    Ok(())
}
