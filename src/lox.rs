use std::io::{stdin, stdout, Result, Write};

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[derive(Default)]
pub struct Lox {
    interpreter: Interpreter,
    // errors: Vec<Box<dyn Error>>,
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

    // pub fn error(&mut self, err: Box<dyn Error>) {
    //     self.errors.push(err);
    // }

    fn run(&self, source: &str) {
        let scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner);
        // let mut errors: Vec<Box<dyn Error>> = Vec::new();

        match parser.parse() {
            Ok(statements) => match self.interpreter.interpret(statements) {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            },
            Err(err) => println!("{}", err),
        }
    }
}
