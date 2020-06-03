use clap::ArgMatches;
use std::fs;
use std::str::Chars;
use std::io::{BufReader, Read, BufRead};
use std::fs::File;
use std::io::Result;
use std::iter::{Scan, Peekable, Fuse};

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
            '0' => Token::U32(0),
            '1' => Token::U32(1),
            '2' => Token::U32(2),
            '3' => Token::U32(3),
            '4' => Token::U32(4),
            '5' => Token::U32(5),
            '6' => Token::U32(6),
            '7' => Token::U32(7),
            '8' => Token::U32(8),
            '9' => Token::U32(9),
            ';' => Token::SemiColon,
            v => panic!("Unable to handle {}", v)
        }
    }
}

#[derive(Debug, Clone)]
enum ScanResult<T> {
    Some(T),
    None,
    Error(ScannerError),
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    U32(u32),
    SemiColon,
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

pub struct ScannerIterator<'a> {
    inner: Peekable<Chars<'a>>,
}

impl<'a> ScannerIterator<'a> {
    fn new(inner: Peekable<Chars<'a>>) -> Self {
        ScannerIterator { inner }
    }

    fn ignore(&self, value: char) -> bool {
        return value == ' ' || '\t' == value || '\n' == value || '\r' == value;
    }

    fn read_token(&mut self) -> Option<Token> {
        while let Some(c) = self.inner.next() {
            if !self.ignore(c) {
                if c.is_digit(10) {
                    return Some(self.read_int_lit_token(c.to_digit(10).unwrap()));
                }

                return Some(Token::from(c));
            }
        }

        return None;
    }


    fn read_int_lit_token(&mut self, token: u32) -> Token {
        let mut result = token;

        while self.inner.peek().map_or_else(|| false, |x| x.is_digit(10)) {
            let next = self.inner.next().unwrap();
            if next.is_digit(10) {
                result = (result * 10) + next.to_digit(10).unwrap()
            } else {
                return Token::U32(result);
            }
        }

        return Token::U32(result);
    }
}

impl<'a> Iterator for ScannerIterator<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        return self.read_token();
    }
}


impl Scanner {
    pub fn from_arg_matches(_arg_matches: &ArgMatches) -> Scanner {
        Scanner::new()
    }

    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn new_iterator<'a>(&self, chars: Chars<'a>) -> ScannerIterator<'a> {
        ScannerIterator::new(chars.peekable())
    }
}

