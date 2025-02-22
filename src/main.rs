use anyhow::Result;
use lamb::eval::eval_expression;
use lamb::parser::{parse_expr, Atom, Expr};
use std::io::{self, Write};

static PROGRAM_NAME: &str = "lamb";
static HELP_MESSAGE: &str = "REPL Usage
    'help' '?'       show this help message
    'exit' 'quit'    finish lamb REPL
";

fn main() -> Result<()> {
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
            _ => (),
        }
        match parse_expr(&input) {
            Ok((_, expr)) => {
                // println!("Input: {input}");
                match eval_expression(expr) {
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
                            _ => eprintln!("Error!"),
                        },
                        _ => eprintln!("Error!"),
                    },
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
    Ok(())
}
