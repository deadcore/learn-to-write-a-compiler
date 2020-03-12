use clap::ArgMatches;
use std::fs;
use std::str::Chars;
use std::io::{BufReader, Read, BufRead, Error};
use std::fs::File;
use std::io::Result;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token: TokenType,
    pub int_value: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Intlit,
}

pub trait Precedence {
    fn precedence(&self) -> u32;
}

impl Precedence for TokenType {
    fn precedence(&self) -> u32 {
        match self {
            TokenType::Plus => 10,
            TokenType::Minus => 10,
            TokenType::Star => 20,
            TokenType::Slash => 20,
            TokenType::Intlit => 0,
        }
    }
}

pub struct Scanner {}

pub struct ScannerIterator {
    inner: BufReader<File>,
    peeked_result: Option<u8>,
}

impl ScannerIterator {
    pub fn new(reader: BufReader<File>, peeked_result: Option<u8>) -> Self {
        ScannerIterator { inner: reader, peeked_result }
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        Self::new(reader, None)
    }

    fn scan_token(&self, token: char) -> Option<Token> {
        if self.ignore(token) {
            return None;
        }
        match token {
            '+' => Some(Token { token: TokenType::Plus, int_value: 0 }),
            '-' => Some(Token { token: TokenType::Minus, int_value: 0 }),
            '*' => Some(Token { token: TokenType::Star, int_value: 0 }),
            '/' => Some(Token { token: TokenType::Slash, int_value: 0 }),
            '0' => Some(Token { token: TokenType::Intlit, int_value: 0 }),
            '1' => Some(Token { token: TokenType::Intlit, int_value: 1 }),
            '2' => Some(Token { token: TokenType::Intlit, int_value: 2 }),
            '3' => Some(Token { token: TokenType::Intlit, int_value: 3 }),
            '4' => Some(Token { token: TokenType::Intlit, int_value: 4 }),
            '5' => Some(Token { token: TokenType::Intlit, int_value: 5 }),
            '6' => Some(Token { token: TokenType::Intlit, int_value: 6 }),
            '7' => Some(Token { token: TokenType::Intlit, int_value: 7 }),
            '8' => Some(Token { token: TokenType::Intlit, int_value: 8 }),
            '9' => Some(Token { token: TokenType::Intlit, int_value: 9 }),
            v => {
                panic!("Unable to handle {}", v);
            }
        }
    }

    fn ignore(&self, value: char) -> bool {
        return value == ' ' || '\t' == value || '\n' == value || '\r' == value;
    }

    fn read_byte_from_buffer(&mut self) -> Result<Option<u8>> {
        let mut buffer = [0; 1];

        self.inner.read(&mut buffer)
            .map(|read| {
                if read != 0 {
                    Some(buffer[0])
                } else {
                    None
                }
            })
    }

    fn read_byte(&mut self) -> Result<Option<u8>> {
        match self.peeked_result {
            Some(byte) => {
                self.peeked_result = None;
                return Result::Ok(Some(byte));
            }
            None => self.read_byte_from_buffer()
        }
    }

    fn take_peek_result(&mut self) -> Option<u8> {
        let t = self.peeked_result;
        self.peeked_result = None;
        t
    }

    fn peek_byte(&mut self) -> Option<u8> {
        // Return either the currently cached peeked byte or obtain a new one
        // from the underlying reader.
        match self.peeked_result {
            Some(ref old_res) => Some(old_res.clone()),
            None => {
                // First get the result of the read from the underlying reader
                self.peeked_result = self.read_byte_from_buffer().unwrap();

                // Now just return that
                self.peeked_result
            }
        }
    }
}

impl Iterator for ScannerIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        while let Some(byte) = self.read_byte().unwrap() {
            if let Some(mut token) = self.scan_token(byte as char) {
                match token.token {
                    TokenType::Intlit => {
                        let mut next_token = match self.peek_byte() {
                            Some(next_byte) => self.scan_token(next_byte as char),
                            None => None
                        };
                        while next_token.is_some() && next_token.unwrap().token == TokenType::Intlit {
                            self.take_peek_result();
                            token.int_value = (token.int_value * 10) + next_token.unwrap().int_value;
                            next_token = match self.peek_byte() {
                                Some(next_byte) => self.scan_token(next_byte as char),
                                None => None
                            };
                        }

                        return Some(token);
                    }
                    other => return Some(token)
                }
            }
        };

        None
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

