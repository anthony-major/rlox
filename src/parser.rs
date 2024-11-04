use crate::{
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
}
