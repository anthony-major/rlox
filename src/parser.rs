use std::{error::Error, fmt::Display};

use crate::{
    ast::{
        Assign, Binary, Block, Expr, Expression, Grouping, IfStmt, Literal, Logical, Print, Stmt,
        Unary, Var, Variable,
    },
    interpreter::RuntimeError,
    lox::Lox,
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub type ParserResult<T> = Result<T, Box<dyn Error>>;

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

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
        self.current_token = self.scanner.get_next_token()?;

        let mut statements: Vec<Stmt> = Vec::new();

        while self.current_token.kind() != &TokenKind::Eof {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => Lox::error(err),
            }
        }

        Ok(statements)
    }

    fn synchronize(&mut self) -> ParserResult<()> {
        self.current_token = self.scanner.get_next_token()?;

        while !matches!(self.current_token.kind(), TokenKind::Eof) {
            match self.current_token.kind() {
                TokenKind::Semicolon => {
                    self.current_token = self.scanner.get_next_token()?;
                    return Ok(());
                }
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return Ok(()),
                _ => self.current_token = self.scanner.get_next_token()?,
            }
        }

        Ok(())
    }

    fn declaration(&mut self) -> ParserResult<Stmt> {
        if matches!(self.current_token.kind(), TokenKind::Var) {
            self.current_token = self.scanner.get_next_token()?;

            match self.var_declaration() {
                Ok(statement) => return Ok(statement),
                Err(err) => {
                    self.synchronize()?;
                    return Err(err);
                }
            }
        }

        match self.statement() {
            Ok(statement) => Ok(statement),
            Err(err) => {
                self.synchronize()?;
                Err(err)
            }
        }
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = match self.current_token.kind() {
            TokenKind::Identifier(_) => {
                let temp = self.current_token.clone();
                self.current_token = self.scanner.get_next_token()?;
                temp
            }
            _ => {
                return Err(Box::new(ParserError::new(
                    self.current_token.clone(),
                    "Expected variable name".to_string(),
                )))
            }
        };

        let initializer = match self.current_token.kind() {
            TokenKind::Equal => {
                self.current_token = self.scanner.get_next_token()?;
                Some(Box::new(self.expression()?))
            }
            _ => None,
        };

        match self.current_token.kind() {
            TokenKind::Semicolon => {
                self.current_token = self.scanner.get_next_token()?;
                Ok(Stmt::Var(Var::new(name, initializer)))
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expected ';' after variable declaration".to_string(),
            ))),
        }
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if matches!(self.current_token.kind(), TokenKind::Print) {
            self.current_token = self.scanner.get_next_token()?;

            return self.print_statement();
        }
        if matches!(self.current_token.kind(), TokenKind::LeftBrace) {
            self.current_token = self.scanner.get_next_token()?;

            return Ok(Stmt::Block(Block::new(self.block()?)));
        }
        if matches!(self.current_token.kind(), TokenKind::If) {
            self.current_token = self.scanner.get_next_token()?;

            return self.if_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        let value = self.expression()?;

        match self.current_token.kind() {
            TokenKind::Semicolon => {
                self.current_token = self.scanner.get_next_token()?;
                Ok(Stmt::Print(Print::new(Box::new(value))))
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect ';' after value".to_string(),
            ))),
        }
    }

    fn block(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !matches!(
            self.current_token.kind(),
            TokenKind::RightBrace | TokenKind::Eof
        ) {
            statements.push(self.declaration()?);
        }

        match self.current_token.kind() {
            TokenKind::RightBrace => {
                self.current_token = self.scanner.get_next_token()?;

                Ok(statements)
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect '}' after block".to_string(),
            ))),
        }
    }

    fn if_statement(&mut self) -> ParserResult<Stmt> {
        if !matches!(self.current_token.kind(), TokenKind::LeftParen) {
            return Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect '(' after if".to_string(),
            )));
        }
        self.current_token = self.scanner.get_next_token()?;

        let condition = self.expression()?;

        if !matches!(self.current_token.kind(), TokenKind::RightParen) {
            return Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect ')' after if condition".to_string(),
            )));
        }
        self.current_token = self.scanner.get_next_token()?;

        let then_branch = self.statement()?;
        let else_branch = match self.current_token.kind() {
            TokenKind::Else => {
                self.current_token = self.scanner.get_next_token()?;

                Some(self.statement()?)
            }
            _ => None,
        };

        Ok(Stmt::IfStmt(IfStmt::new(
            Box::new(condition),
            Box::new(then_branch),
            match else_branch {
                Some(stmt) => Some(Box::new(stmt)),
                None => None,
            },
        )))
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;

        match self.current_token.kind() {
            TokenKind::Semicolon => {
                self.current_token = self.scanner.get_next_token()?;
                Ok(Stmt::Expression(Expression::new(Box::new(expr))))
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect ';' after expression".to_string(),
            ))),
        }
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.or()?;

        match self.current_token.kind() {
            TokenKind::Equal => {
                let equals = self.current_token.clone();
                self.current_token = self.scanner.get_next_token()?;

                let value = self.assignment()?;

                match expr {
                    Expr::Variable(variable) => {
                        let name = variable.name.clone();

                        Ok(Expr::Assign(Assign::new(name, Box::new(value))))
                    }
                    _ => {
                        Lox::error(Box::new(RuntimeError::new(
                            equals,
                            "Invalid assignment target".to_string(),
                        )));
                        Ok(expr)
                    }
                }
            }
            _ => Ok(expr),
        }
    }

    fn or(&mut self) -> ParserResult<Expr> {
        let mut expr = self.and()?;

        while matches!(self.current_token.kind(), TokenKind::Or) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.and()?;
            expr = Expr::Logical(Logical::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParserResult<Expr> {
        let mut expr = self.equality()?;

        while matches!(self.current_token.kind(), TokenKind::And) {
            let operator = self.current_token.clone();
            self.current_token = self.scanner.get_next_token()?;
            let right = self.equality()?;
            expr = Expr::Logical(Logical::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParserResult<Expr> {
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

    fn comparison(&mut self) -> ParserResult<Expr> {
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

    fn term(&mut self) -> ParserResult<Expr> {
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

    fn factor(&mut self) -> ParserResult<Expr> {
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

    fn unary(&mut self) -> ParserResult<Expr> {
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

    fn primary(&mut self) -> ParserResult<Expr> {
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
            TokenKind::Identifier(_) => {
                let temp = self.current_token.clone();
                self.current_token = self.scanner.get_next_token()?;
                Ok(Expr::Variable(Variable::new(temp)))
            }
            _ => Err(Box::new(ParserError::new(
                self.current_token.clone(),
                "Expect expression".to_string(),
            ))),
        }
    }
}
