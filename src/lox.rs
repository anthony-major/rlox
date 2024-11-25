use std::{
    error::Error,
    io::{stdin, stdout, Result, Write},
};

use crate::{
    interpreter::Interpreter,
    scanner::{Scanner, ScannerError},
};

#[derive(Default)]
pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn run_file(&self, path: &str) -> Result<()> {
        let code = std::fs::read_to_string(path)?;

        self.run(&code);

        Ok(())
    }

    pub fn run_prompt(&self) -> Result<()> {
        let mut input = String::new();

        loop {
            print!(">");
            stdout().flush()?;
            input.clear();
            let bytes_read = stdin().read_line(&mut input)?;
            if bytes_read == 0 {
                break;
            }

            self.run(&input);
        }

        Ok(())
    }

    fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let mut errors: Vec<Box<dyn Error>> = Vec::new();

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
