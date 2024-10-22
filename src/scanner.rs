use std::{error::Error, fmt::Display};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct ScannerError {
    kind: ScannerErrorKind,
    line: usize,
}

#[derive(Debug)]
pub enum ScannerErrorKind {
    InvalidCharacter(char),
    UnterminatedString,
}

impl ScannerError {
    pub fn new(kind: ScannerErrorKind, line: usize) -> Self {
        Self {
            kind: kind,
            line: line,
        }
    }
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ScannerErrorKind::InvalidCharacter(c) => {
                write!(
                    f,
                    "Invalid character found while scanning at line {}: {}",
                    self.line, c
                )
            }
            ScannerErrorKind::UnterminatedString => {
                write!(f, "Encountered unterminated string at line {}", self.line)
            }
        }
    }
}

pub struct Scanner {
    source: Vec<char>,
    index: usize,
    line: usize,
    current_char: Option<char>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut new_self = Self {
            source: source.chars().collect(),
            index: 0,
            line: 1,
            current_char: None,
        };

        if !new_self.source.is_empty() {
            new_self.current_char = Some(new_self.source[new_self.index]);
        }

        new_self
    }

    pub fn get_next_token(&mut self) -> Result<Token, ScannerError> {
        if self.current_char.is_none() {
            return Ok(Token::new(TokenKind::Eof, self.line));
        }

        match self.current_char.unwrap() {
            '(' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::LeftParen, self.line))
            }
            ')' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::RightParen, self.line))
            }
            '{' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::LeftBrace, self.line))
            }
            '}' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::RightBrace, self.line))
            }
            ',' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Comma, self.line))
            }
            '.' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Dot, self.line))
            }
            '-' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Minus, self.line))
            }
            '+' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Plus, self.line))
            }
            ';' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Semicolon, self.line))
            }
            '*' => {
                self.get_next_character();
                Ok(Token::new(TokenKind::Star, self.line))
            }
            '!' => match self.peek_next_character() {
                Some('=') => {
                    self.get_next_character();
                    self.get_next_character(); // Once for '!' and once for '='
                    Ok(Token::new(TokenKind::BangEqual, self.line))
                }
                _ => {
                    self.get_next_character();
                    Ok(Token::new(TokenKind::Bang, self.line))
                }
            },
            '=' => match self.peek_next_character() {
                Some('=') => {
                    self.get_next_character();
                    self.get_next_character(); // Once for each '='
                    Ok(Token::new(TokenKind::EqualEqual, self.line))
                }
                _ => {
                    self.get_next_character();
                    Ok(Token::new(TokenKind::Equal, self.line))
                }
            },
            '<' => match self.peek_next_character() {
                Some('=') => {
                    self.get_next_character();
                    self.get_next_character(); // Once for '<' and once for '='
                    Ok(Token::new(TokenKind::LessEqual, self.line))
                }
                _ => {
                    self.get_next_character();
                    Ok(Token::new(TokenKind::Less, self.line))
                }
            },
            '>' => match self.peek_next_character() {
                Some('=') => {
                    self.get_next_character();
                    self.get_next_character(); // Once for '>' and once for '='
                    Ok(Token::new(TokenKind::GreaterEqual, self.line))
                }
                _ => {
                    self.get_next_character();
                    Ok(Token::new(TokenKind::Greater, self.line))
                }
            },
            '/' => match self.peek_next_character() {
                Some('/') => {
                    self.get_next_character();
                    self.get_next_character(); // Once for each '/'
                    self.skip_comment();
                    self.get_next_token()
                }
                _ => {
                    self.get_next_character();
                    Ok(Token::new(TokenKind::Slash, self.line))
                }
            },
            '\n' => {
                self.line += 1;
                self.get_next_character();
                self.get_next_token()
            }
            ' ' | '\r' | '\t' => {
                self.get_next_character();
                self.get_next_token()
            }
            '"' => {
                self.get_next_character();
                self.get_string()
            }
            c if c.is_ascii_digit() => Ok(self.get_number()),
            c if c.is_ascii_alphabetic() || c == '_' => Ok(self.get_id()),
            _ => {
                let invalid_char = self.current_char.unwrap();
                self.get_next_character();
                Err(ScannerError::new(
                    ScannerErrorKind::InvalidCharacter(invalid_char),
                    self.line,
                ))
            }
        }
    }

    fn get_next_character(&mut self) {
        self.index += 1;

        if self.index >= self.source.len() {
            self.current_char = None;
            return;
        }

        self.current_char = Some(self.source[self.index]);
    }

    fn peek_next_character(&self) -> Option<char> {
        if self.index + 1 >= self.source.len() {
            return None;
        }

        Some(self.source[self.index + 1])
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            self.get_next_character();
        }
    }

    fn get_string(&mut self) -> Result<Token, ScannerError> {
        let mut string_chars: Vec<char> = Vec::new();

        while self.current_char.is_some() && self.current_char.unwrap() != '"' {
            string_chars.push(self.current_char.unwrap());
            self.get_next_character();
        }

        if self.current_char.is_none() {
            return Err(ScannerError::new(
                ScannerErrorKind::UnterminatedString,
                self.line,
            ));
        }

        self.get_next_character();

        let token_string: String = string_chars.into_iter().collect();

        Ok(Token::new(TokenKind::String(token_string), self.line))
    }

    fn get_number(&mut self) -> Token {
        let mut number_chars: Vec<char> = Vec::new();

        while self.current_char.is_some() && self.current_char.unwrap().is_ascii_digit() {
            number_chars.push(self.current_char.unwrap());
            self.get_next_character();
        }

        if self.current_char.is_some()
            && self.current_char.unwrap() == '.'
            && self.peek_next_character().is_some()
            && self.peek_next_character().unwrap().is_ascii_digit()
        {
            number_chars.push('.');
            self.get_next_character();
        } else {
            let number = number_chars
                .into_iter()
                .collect::<String>()
                .parse::<f64>()
                .unwrap();

            return Token::new(TokenKind::Number(number), self.line);
        }

        while self.current_char.is_some() && self.current_char.unwrap().is_ascii_digit() {
            number_chars.push(self.current_char.unwrap());
            self.get_next_character();
        }

        let number = number_chars
            .into_iter()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        Token::new(TokenKind::Number(number), self.line)
    }

    fn get_id(&mut self) -> Token {
        let mut id: Vec<char> = Vec::new();

        while self.current_char.is_some() {
            if !self.current_char.unwrap().is_ascii_alphanumeric()
                && self.current_char.unwrap() != '_'
            {
                break;
            }

            id.push(self.current_char.unwrap());

            self.get_next_character();
        }

        let id: String = id.into_iter().collect();

        match id.as_str() {
            "and" => Token::new(TokenKind::And, self.line),
            "class" => Token::new(TokenKind::Class, self.line),
            "else" => Token::new(TokenKind::Else, self.line),
            "false" => Token::new(TokenKind::False, self.line),
            "for" => Token::new(TokenKind::For, self.line),
            "fun" => Token::new(TokenKind::Fun, self.line),
            "if" => Token::new(TokenKind::If, self.line),
            "nil" => Token::new(TokenKind::Nil, self.line),
            "or" => Token::new(TokenKind::Or, self.line),
            "print" => Token::new(TokenKind::Print, self.line),
            "return" => Token::new(TokenKind::Return, self.line),
            "super" => Token::new(TokenKind::Super, self.line),
            "this" => Token::new(TokenKind::This, self.line),
            "true" => Token::new(TokenKind::True, self.line),
            "var" => Token::new(TokenKind::Var, self.line),
            "while" => Token::new(TokenKind::While, self.line),
            _ => Token::new(TokenKind::Identifier(id), self.line),
        }
    }
}
