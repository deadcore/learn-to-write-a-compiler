use clap::ArgMatches;
use std::fs;
use std::str::Chars;
use std::io::{BufReader, Read, BufRead};
use std::fs::File;
use std::io::Result;
use std::iter::Scan;

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
            '0' => Token::Intlit(0),
            '1' => Token::Intlit(1),
            '2' => Token::Intlit(2),
            '3' => Token::Intlit(3),
            '4' => Token::Intlit(4),
            '5' => Token::Intlit(5),
            '6' => Token::Intlit(6),
            '7' => Token::Intlit(7),
            '8' => Token::Intlit(8),
            '9' => Token::Intlit(9),
            // ';' => Some(Token { token: TokenType::SemiColon, int_value: 0 }),
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
    Intlit(i32),
}

pub trait Precedence {
    fn precedence(&self) -> u32;
}

impl Precedence for TokenType {
    fn precedence(&self) -> u32 {
        match self {
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Star => 20,
            Token::Slash => 20,
            Token::Intlit => 0,
        }
    }
}

pub struct Scanner {}

pub struct ScannerIterator {
    inner: BufReader<File>,
    peeked_result: ScanResult<u8>,
}

impl ScannerIterator {
    fn new(reader: BufReader<File>, peeked_result: ScanResult<u8>) -> Self {
        ScannerIterator { inner: reader, peeked_result }
    }

    fn open(path: impl AsRef<std::path::Path>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        Self::new(reader, ScanResult::None)
    }

    fn ignore(&self, value: char) -> bool {
        return value == ' ' || '\t' == value || '\n' == value || '\r' == value;
    }

    fn read_byte_from_buffer(&mut self) -> ScanResult<u8> {
        let mut buffer = [0; 1];

        match self.inner.read(&mut buffer) {
            std::io::Result::Ok(bytes_read) if bytes_read == 0 => ScanResult::None,
            std::io::Result::Ok(bytes_read) => ScanResult::Some(buffer[0]),
            std::io::Result::Err(err) => ScanResult::Error(ScannerError::from(err))
        }
    }

    fn read_byte(&mut self) -> ScanResult<u8> {
        match self.peeked_result.clone() {
            ScanResult::Some(byte) => {
                self.peeked_result = ScanResult::None;
                return ScanResult::Some(byte);
            }
            ScanResult::None => self.read_byte_from_buffer(),
            other => other
        }
    }

    fn read_char(&mut self) -> ScanResult<char> {
        match self.read_byte() {
            ScanResult::Some(value) => ScanResult::Some(value as char),
            ScanResult::Error(err) => ScanResult::Error(err),
            ScanResult::None => ScanResult::None,
        }
    }

    fn read_token(&mut self) -> ScanResult<Token> {
        let mut result: Option<Token> = None;

        while result.is_none() {
            match self.read_char() {
                ScanResult::None => return ScanResult::None,
                ScanResult::Error(err) => return ScanResult::Error(err),
                ScanResult::Some(char)  if self.ignore(char) => {}
                ScanResult::Some(char) => return ScanResult::Some(Token::from(char))
            }
        }

        match result {
            Some(token) => ScanResult::Some(token),
            None => ScanResult::None,
        }
    }

    fn peek_byte(&mut self) -> ScanResult<u8> {
        match self.peeked_result.clone() {
            ScanResult::Some(t) => ScanResult::Some(t),
            ScanResult::None => {
                // First get the result of the read from the underlying reader
                self.peeked_result = self.read_byte_from_buffer();

                // Now just return that
                self.peeked_result.clone()
            }
            other => other
        }
    }

    fn clear_peek_result(&mut self) {
        self.peeked_result = ScanResult::None;
    }

    fn peek_char(&mut self) -> ScanResult<char> {
        match self.peek_byte() {
            ScanResult::Some(value) => ScanResult::Some(value as char),
            ScanResult::Error(err) => ScanResult::Error(err),
            ScanResult::None => ScanResult::None,
        }
    }

    fn read_int_lit_token(&mut self, token: Token) -> Token {
        let mut result = token;

        while let ScanResult::Some(next_token) = self.peek_token() {
            match next_token.token {
                Token::Intlit(v) => {
                    self.clear_peek_result();
                    result.int_value = (result.int_value * 10) + next_token.int_value;
                }
                other => break
            }
        };

        return result
    }

    fn peek_token(&mut self) -> ScanResult<Token> {
        match self.read_char() {
            ScanResult::None => return ScanResult::None,
            ScanResult::Error(err) => return ScanResult::Error(err),
            ScanResult::Some(char)  if self.ignore(char) => ScanResult::None,
            ScanResult::Some(char) => ScanResult::Some(Token::from(char))
        }
    }
}

impl Iterator for ScannerIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let token = self.read_token();

        match token {
            ScanResult::Some(token) if token.token == TokenType::Intlit => {
                Some(self.read_int_lit_token(token))
            }
            ScanResult::Some(token) => Some(token),
            ScanResult::Error(err) => panic!(err),
            ScanResult::None => None,
        }
    }
}


impl Scanner {
    pub fn from_arg_matches(_arg_matches: &ArgMatches) -> Scanner {
        Scanner::new()
    }

    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn new_iterator(&self, filename: &str) -> ScannerIterator {
        ScannerIterator::open(filename)
    }
}

