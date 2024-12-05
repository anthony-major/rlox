use std::process::ExitCode;

use ast::{Binary, Expr, Grouping, Literal, Unary};
// use ast_printer::AstPrinter;
use interpreter::Interpreter;
use lox::Lox;
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenKind};

mod ast;
// mod ast_printer;
mod environment;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod token;

fn main() -> ExitCode {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        return ExitCode::FAILURE;
    } else if args.len() == 2 {
        let mut lox = Lox::default();

        match lox.run_file(&args[1]) {
            Ok(_) => return ExitCode::SUCCESS,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        }
    } else {
        let mut lox = Lox::default();

        match lox.run_prompt() {
            Ok(_) => return ExitCode::SUCCESS,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        }
    }
}
