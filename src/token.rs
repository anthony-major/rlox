use std::{fmt::Display, hash::Hash};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Token {
    kind: TokenKind,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self {
            kind: kind,
            line: line,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn line(&self) -> &usize {
        &self.line
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token<{}:{}>", self.kind, self.line)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Eq for TokenKind {}

impl Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Number(x) => (*x as u64).hash(state),
            _ => std::mem::discriminant(self).hash(state),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_string = match self {
            Self::And => "and",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Class => "class",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Else => "else",
            Self::Eof => "eof",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::False => "false",
            Self::For => "for",
            Self::Fun => "fun",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Identifier(id) => &format!("Identifier:{}", id),
            Self::If => "if",
            Self::LeftBrace => "{",
            Self::LeftParen => "(",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Minus => "-",
            Self::Nil => "nil",
            Self::Number(x) => &format!("Number:{}", x),
            Self::Or => "or",
            Self::Plus => "+",
            Self::Print => "print",
            Self::Return => "return",
            Self::RightBrace => "}",
            Self::RightParen => ")",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::Star => "*",
            Self::String(str) => &format!("\"{}\"", str),
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
        };

        write!(f, "{}", token_string)
    }
}
