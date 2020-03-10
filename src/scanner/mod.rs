use clap::ArgMatches;

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

impl Scanner {
    pub fn from_arg_matches(_arg_matches: &ArgMatches) -> Scanner {
        Scanner::new()
    }

    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan(&self, token: char) -> Option<Token> {
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
}

