use std::process::ExitCode;

use ast::{Binary, Expr, Grouping, Literal, Unary};
use interpreter::Interpreter;
use token::{Token, TokenKind};

mod ast;
mod interpreter;
mod scanner;
mod token;

fn main() {
    let expression = Expr::Binary(Box::new(Binary::new(left, operator, right)));
}

// fn main() -> ExitCode {
//     let args = std::env::args().collect::<Vec<String>>();

//     if args.len() > 2 {
//         println!("Usage: rlox [script]");
//         return ExitCode::FAILURE;
//     } else if args.len() == 2 {
//         match Interpreter::run_file(&args[1]) {
//             Ok(_) => return ExitCode::SUCCESS,
//             Err(err) => {
//                 println!("{}", err);
//                 return ExitCode::FAILURE;
//             }
//         }
//     } else {
//         match Interpreter::run_prompt() {
//             Ok(_) => return ExitCode::SUCCESS,
//             Err(err) => {
//                 println!("{}", err);
//                 return ExitCode::FAILURE;
//             }
//         }
//     }
// }
