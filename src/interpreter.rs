use std::io::Write;

use crate::{
    scanner::{Scanner, ScannerError},
    token::TokenKind,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn run_file(path: &str) -> std::io::Result<()> {
        let interpreter = Self {};
        let code = std::fs::read_to_string(path)?;

        interpreter.run(&code);

        Ok(())
    }

    pub fn run_prompt() -> std::io::Result<()> {
        let interpreter = Self {};
        let mut input = String::new();

        loop {
            print!(">");
            std::io::stdout().flush()?;
            let bytes_read = std::io::stdin().read_line(&mut input)?;
            if bytes_read == 0 {
                break;
            }

            interpreter.run(&input);

            input.clear();
        }

        Ok(())
    }

    fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let mut errors: Vec<ScannerError> = Vec::new();

        loop {
            match scanner.get_next_token() {
                Ok(token) => {
                    println!("{}", token);

                    if token.kind() == &TokenKind::Eof {
                        break;
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        for err in errors {
            println!("{}", err);
        }
    }
}
