use crate::{
    ast::{Binary, Expr, Grouping, Literal, Unary},
    scanner::Scanner,
    token::{Token, TokenKind},
};

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

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while matches!(
            self.current_token.kind(),
            TokenKind::BangEqual | TokenKind::EqualEqual
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token().unwrap();
            let right = self.comparison();
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while matches!(
            self.current_token.kind(),
            TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token().unwrap();
            let right = self.term();
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while matches!(
            self.current_token.kind(),
            TokenKind::Minus | TokenKind::Plus
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token().unwrap();
            let right = self.factor();
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while matches!(
            self.current_token.kind(),
            TokenKind::Slash | TokenKind::Star
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token().unwrap();
            let right = self.unary();
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if matches!(
            self.current_token.kind(),
            TokenKind::Bang | TokenKind::Minus
        ) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token().unwrap();
            let right = self.unary();
            return Expr::Unary(Unary::new(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        match self.current_token.kind() {
            TokenKind::True | TokenKind::False | TokenKind::Nil => {
                let token = self.current_token.clone();
                self.current_token = self.scanner.get_next_token().unwrap();
                Expr::Literal(Literal::new(token))
            }
            TokenKind::LeftParen => {
                self.current_token = self.scanner.get_next_token().unwrap();
                let expr = self.expression();
                if !matches!(self.current_token.kind(), TokenKind::RightParen) {
                    todo!() // Error here
                }
                self.current_token = self.scanner.get_next_token().unwrap();
                Expr::Grouping(Grouping::new(Box::new(expr)))
            }
            _ => Expr::Literal(Literal::new(Token::new(TokenKind::Number(0.0), 0))),
        }
    }
}
