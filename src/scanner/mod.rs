use std::convert::TryFrom;
use std::iter::Peekable;
use std::str::Chars;

use log::{debug, trace};

#[derive(Debug, Clone)]
pub enum ScannerError {
    Error(String),
}

impl std::convert::From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        ScannerError::Error(format!("{}", err))
    }
}

impl std::convert::From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            ';' => Token::SemiColon,
            '=' => Token::Assignment,
            '\n' => Token::NewLine,
            ' ' => Token::Space,
            v => panic!("Unable to handle token: [{}]", v)
        }
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
    NewLine
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
            v => Err(format!("Unable to handle KeywordToken: [{}]", value))
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
            _ => 0
        }
    }
}

pub struct Scanner {}

pub struct TokenIterator<T: Iterator<Item=char>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=char>> TokenIterator<T> {
    fn new(inner: Peekable<T>) -> Self {
        TokenIterator { inner }
    }

    fn ignore(&self, value: char) -> bool {
        return value == ' ' || '\t' == value || '\n' == value || '\r' == value;
    }

    fn read_token(&mut self) -> Option<Token> {
        if let Some(&c) = self.inner.peek() {
            if c.is_digit(10) {
                return self.read_int_lit_token();
            }
            if c.is_alphabetic() {
                return self.read_alphbetic_token();
            }
            return self.read_symbol();
        }

        return None;
    }

    fn read_symbol(&mut self) -> Option<Token> {
        if let Some(t) = self.inner.next() {
            return Some(Token::from(t));
        }
        panic!("Error - Received no token but expected a whitespace")
    }

    fn read_alphbetic_token(&mut self) -> Option<Token> {
        let mut result = String::new();
        while self.inner.peek().map_or_else(|| false, |x| x.is_alphanumeric()) {
            let next = self.inner.next().unwrap();
            result.push(next)
        }

        match KeywordToken::try_from(result.as_str()) {
            Ok(v) => Some(Token::Keyword(v)),
            Err(v) => Some(Token::Identifier(result)),
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
        let token = self.read_token();
        trace!("Token: {:?}", token);
        return token;
    }
}


impl Scanner {
    pub fn from_arg_matches() -> Scanner {
        Scanner::new()
    }

    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn new_iterator<T: Iterator<Item=char>>(&self, chars: T) -> TokenIterator<T> {
        TokenIterator::new(chars.peekable())
    }
}

