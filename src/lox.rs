use std::{
    cell::RefCell,
    error::Error,
    io::{stdin, stdout, Result, Write},
    rc::Rc,
};

use crate::{interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner};

#[derive(Default)]
pub struct Lox {
    interpreter: Rc<RefCell<Interpreter>>,
}

impl Lox {
    pub fn error(err: Box<dyn Error>) {
        println!("{}", err);
    }

    pub fn run_file(&mut self, path: &str) -> Result<()> {
        let code = std::fs::read_to_string(path)?;

        self.run(&code);

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let mut input = String::new();

        loop {
            print!(">");
            stdout().flush()?;
            input.clear();
            let bytes_read = stdin().read_line(&mut input)?;
            if bytes_read == 0 {
                break;
            }

            self.run(&input.trim_end());
        }

        Ok(())
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner);
        // let mut errors: Vec<Box<dyn Error>> = Vec::new();

        match parser.parse() {
            Ok(statements) => {
                let mut resolver = Resolver::new(self.interpreter.clone());
                resolver.resolve(&statements);

                match self.interpreter.borrow_mut().interpret(statements) {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
