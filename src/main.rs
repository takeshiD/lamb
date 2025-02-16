use lamb::parser;
use std::io::{self, Write};

static PROGRAM_NAME: &str = "lamb";
static HELP_MESSAGE: &str = "REPL Usage
    'help' '?'       show this help message
    'exit' 'quit'    finish lamb REPL
";

fn main() {
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
        match parser::parse_expr(&input) {
            Ok((_, expr)) => {
                println!("Input: {input}\nExpr: {expr:#?}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}
