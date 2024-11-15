use std::{error::Error, fmt::Display};

use crate::{
    ast::{Binary, Expr, Grouping, Literal, Unary},
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub type ParserResult = Result<Expr, Box<dyn Error>>;

#[derive(Debug)]
pub struct ParserError {
    token: Token,
    message: String,
}

impl ParserError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {} at '{}': {}",
            self.token.line(),
            self.token.kind(),
            self.message
        )
    }
}

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Self {
            scanner: scanner,
            current_token: Token::new(TokenKind::Eof, 0),
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        self.current_token = self.scanner.get_next_token()?;

        self.expression()
    }

    fn expression(&mut self) -> ParserResult {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult {
        let mut expr = self.comparison()?;

        while matches!(
            self.current_token.kind(),
            TokenKind::BangEqual | TokenKind::EqualEqual
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult {
        let mut expr = self.term()?;

        while matches!(
            self.current_token.kind(),
            TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParserResult {
        let mut expr = self.factor()?;

        while matches!(
            self.current_token.kind(),
            TokenKind::Minus | TokenKind::Plus
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult {
        let mut expr = self.unary()?;

        while matches!(
            self.current_token.kind(),
            TokenKind::Slash | TokenKind::Star
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult {
        if matches!(
            self.current_token.kind(),
            TokenKind::Bang | TokenKind::Minus
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(operator, Box::new(right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParserResult {
        match self.current_token.kind() {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Nil
            | TokenKind::Number(_)
            | TokenKind::String(_) => {
                let token = self.current_token.clone();
                self.current_token = self.scanner.get_next_token()?;
                Ok(Expr::Literal(Literal::new(token)))
            }
            TokenKind::LeftParen => {
                self.current_token = self.scanner.get_next_token()?;
                let expr = self.expression()?;
                if !matches!(self.current_token.kind(), TokenKind::RightParen) {
                    return Err(Box::new(ParserError::new(
                        self.current_token.clone(),
                        "Expect ')' after expression".to_string(),
                    )));
                }
                self.current_token = self.scanner.get_next_token()?;
                Ok(Expr::Grouping(Grouping::new(Box::new(expr))))
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect expression".to_string(),
            ))),
        }
    }
}
