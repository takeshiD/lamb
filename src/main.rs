use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, BasicHistory, Input};
use lamb::eval::{eval, Environment};
use lamb::parser::{parse_expr, Atom, Expr};

static PROGRAM_NAME: &str = "lamb";
static HELP_MESSAGE: &str = "REPL Usage
    'help' '?'       show this help message
    'exit' 'quit'    finish lamb REPL
";

fn main() -> Result<()> {
    // 環境を作成
    let mut env = Environment::new();

    // 基本的な演算子を環境に追加
    env.add(
        "+".to_string(),
        Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Plus)),
    );
    env.add(
        "-".to_string(),
        Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Minus)),
    );
    env.add(
        "*".to_string(),
        Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Times)),
    );
    env.add(
        "/".to_string(),
        Expr::SelfEvaluation(Atom::Operater(lamb::parser::BuiltinOp::Divide)),
    );
    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{PROGRAM_NAME}>"))
            .history_with(&mut history)
            .interact_text()
        {
            match cmd.as_str() {
                "exit" | "quit" => {
                    println!("Terminated {PROGRAM_NAME}");
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
                _ => match parse_expr(&cmd) {
                    Ok((_, expr)) => match eval(expr, &mut env) {
                        Ok(e) => match e {
                            Expr::SelfEvaluation(atom) => match atom {
                                Atom::Num(n) => println!("{n}"),
                                Atom::Boolean(b) => {
                                    if b {
                                        println!("#t")
                                    } else {
                                        println!("#f")
                                    }
                                }
                                Atom::Operater(_) => println!("<procedure>"),
                            },
                            _ => println!("Result: {:#?}", e),
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    },
                    Err(e) => {
                        println!("Parse error: {:?}", e);
                    }
                },
            }
        }
    }
    Ok(())
}
