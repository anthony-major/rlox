use std::process::ExitCode;

use interpreter::Interpreter;

mod interpreter;
mod scanner;
mod token;

fn main() -> ExitCode {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        return ExitCode::FAILURE;
    } else if args.len() == 2 {
        match Interpreter::run_file(&args[1]) {
            Ok(_) => return ExitCode::SUCCESS,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        }
    } else {
        match Interpreter::run_prompt() {
            Ok(_) => return ExitCode::SUCCESS,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        }
    }
}
