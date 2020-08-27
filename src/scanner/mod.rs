use std::convert::TryFrom;
use std::iter::Peekable;

use log::debug;

#[derive(Debug, Clone)]
pub enum ScannerError {
    Error(String),
}

impl std::convert::From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        ScannerError::Error(format!("{}", err))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    U32(u32),
    SemiColon,
    Keyword(KeywordToken),
    Space,
    Identifier(String),
    Assignment,
    NewLine,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    NotEqual,
    Equality,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordToken {
    Print,
    Int,
}

impl std::convert::TryFrom<&str> for KeywordToken {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "print" => Ok(KeywordToken::Print),
            "int" => Ok(KeywordToken::Int),
            v => Err(format!("Unable to handle KeywordToken: [{}]", v))
        }
    }
}

pub trait Precedence {
    fn precedence(&self) -> u32;
}

impl Precedence for Token {
    fn precedence(&self) -> u32 {
        match self {
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Star => 20,
            Token::Slash => 20,
            Token::LessThan => 40,
            Token::LessThanEqual => 40,
            Token::GreaterThan => 40,
            Token::GreaterThanEqual => 40,
            Token::NotEqual => 40,
            Token::Equality => 40,
            _ => 0
        }
    }
}

pub struct TokenIterator<T: Iterator<Item=char>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=char>> TokenIterator<T> {
    pub fn new_iterator(chars: T) -> TokenIterator<T> {
        TokenIterator::new(chars.peekable())
    }

    fn new(inner: Peekable<T>) -> Self {
        TokenIterator { inner }
    }

    fn read_symbol(&mut self) -> Option<Token> {
        if let Some(t) = self.inner.next() {
            return Some(match t {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                ';' => Token::SemiColon,
                '=' => {
                    match self.inner.peek() {
                        Some('=') => {
                            self.inner.next();
                            Token::Equality
                        }
                        _ => Token::Assignment
                    }
                }
                '\n' => Token::NewLine,
                ' ' => Token::Space,
                '<' => match self.inner.peek() {
                    Some('=') => {
                        self.inner.next();
                        Token::LessThanEqual
                    }
                    _ => Token::LessThan
                }
                '>' => match self.inner.peek() {
                    Some('=') => {
                        self.inner.next();
                        Token::GreaterThanEqual
                    }
                    _ => Token::GreaterThan
                },
                '!' => {
                    match self.inner.next() {
                        Some('=') => Token::NotEqual,
                        None => panic!("Expected another token after ! but got nothing"),
                        Some(v) => panic!("Unsupported token [{}] after !", v),
                    }
                }
                v => panic!("Unable to handle token: [{}]", v)
            });
        }
        panic!("Error - Received no token but expected a whitespace")
    }

    fn read_alphabetic_token(&mut self) -> Option<Token> {
        let mut result = String::new();
        while self.inner.peek().map_or_else(|| false, |x| x.is_alphanumeric()) {
            let next = self.inner.next().unwrap();
            result.push(next)
        }

        match KeywordToken::try_from(result.as_str()) {
            Ok(v) => Some(Token::Keyword(v)),
            Err(v) => {
                debug!("Error while reading the keyword [{:?}], defaulting to identifier", v);
                Some(Token::Identifier(result))
            }
        }
    }

    fn read_int_lit_token(&mut self) -> Option<Token> {
        let mut result = 0;

        while self.inner.peek().map_or_else(|| false, |x| x.is_digit(10)) {
            let next = self.inner.next().unwrap();
            result = (result * 10) + next.to_digit(10).unwrap()
        }

        return Some(Token::U32(result));
    }
}

impl<T: Iterator<Item=char>> Iterator for TokenIterator<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some(&c) = self.inner.peek() {
            debug!("Peeked a char: [{}]", c);
            if c.is_digit(10) {
                return self.read_int_lit_token();
            }
            if c.is_alphabetic() {
                return self.read_alphabetic_token();
            }
            return self.read_symbol();
        }

        return None;
    }
}

