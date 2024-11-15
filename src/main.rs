use std::process::ExitCode;

use ast::{Binary, Expr, Grouping, Literal, Unary};
use ast_printer::AstPrinter;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenKind};

mod ast;
mod ast_printer;
mod interpreter;
mod parser;
mod scanner;
mod token;

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

// fn main() {
//     let expression = Expr::Binary(Binary::new(
//         Box::new(Expr::Unary(Unary::new(
//             Token::new(TokenKind::Minus, 1),
//             Box::new(Expr::Literal(Literal::new(Token::new(
//                 TokenKind::Number(123.0),
//                 1,
//             )))),
//         ))),
//         Token::new(TokenKind::Star, 1),
//         Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
//             Literal::new(Token::new(TokenKind::Number(45.67), 1)),
//         ))))),
//     ));

//     let printer = AstPrinter::default();

//     printer.print(expression);
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return;
    }

    let scanner = Scanner::new(args[1].as_str());
    let mut parser = Parser::new(scanner);

    let expression = parser.parse();

    match expression {
        Ok(expr) => {
            let ast_printer = AstPrinter::default();

            ast_printer.print(expr);
        }
        Err(err) => println!("{}", err),
    }
}
