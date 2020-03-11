use std::borrow::BorrowMut;
use std::fs;
use std::io::{self, Stdout, Write};
use std::iter::Peekable;
use std::slice::Iter;

use clap::ArgMatches;
use log::info;

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpreamble, cgsub, cgpostamble, cgprintint};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::scanner::{Precedence, Scanner, Token, TokenType};
use std::fs::File;

/// An error as returned by a `Handler` method.
#[derive(Debug)]
pub enum Error {
    Error(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Compiler {
    scanner: Scanner
}

impl Compiler {
    pub fn from_arg_matches(arg_matches: &ArgMatches) -> Compiler {
        Compiler::new(
            Scanner::from_arg_matches(arg_matches)
        )
    }

    fn new(scanner: Scanner) -> Compiler {
        Compiler { scanner }
    }

    pub fn compile(&self, filename: &str) -> Result<()> {
        let file = fs::read_to_string(filename)?;
        let staged_tokens: Vec<Token> = file
            .chars()
            .flat_map(|token| self.scanner.scan(token))
            .collect();

        let tokens = self.coalesce_token_sequence(staged_tokens);
        let ast = self.compile_to_ast(&tokens);

        let mut out = File::create("out.s")?;
        let mut registers = Registers::new();

        cgpreamble(out.by_ref());
        let reg = self.interpret_ast_to_asm(out.by_ref(), &mut registers, ast);
        cgprintint(reg, out.by_ref());
        cgpostamble(out.by_ref());

        Ok(())
    }

    fn interpret_ast_to_asm(&self, w: &mut dyn Write, registers: &mut Registers, ast: ASTNode) -> RegisterIndex {
        return match (ast.left, ast.op, ast.right) {
            (Some(left), AstnodeType::Add, Some(right)) => cgadd(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            (Some(left), AstnodeType::Subtract, Some(right)) => cgsub(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            (Some(left), AstnodeType::Multiply, Some(right)) => cgmul(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            (Some(left), AstnodeType::Divide, Some(right)) => cgdiv(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            (l, AstnodeType::Intlit, r) => cgload(ast.int_value, registers, w),
            _ => panic!("Unhandled")
        };
    }

    fn interpret_ast(&self, ast: ASTNode) -> i32 {
        let mut leftval = 0;
        let mut rightval = 0;

        // Get the left and right sub-tree values
        if let Some(left) = ast.left {
            leftval = self.interpret_ast(*left);
        }

        if let Some(right) = ast.right {
            rightval = self.interpret_ast(*right);
        }

        return match ast.op {
            AstnodeType::Add => leftval + rightval,
            AstnodeType::Subtract => leftval - rightval,
            AstnodeType::Multiply => leftval * rightval,
            AstnodeType::Divide => leftval / rightval,
            AstnodeType::Intlit => ast.int_value,
        };
    }

    fn coalesce_token_sequence(&self, staged_tokens: Vec<Token>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < staged_tokens.len() {
            let mut token = staged_tokens[i];
            if token.token == TokenType::Intlit {
                if i + 1 < staged_tokens.len() {
                    for j in (i + 1)..staged_tokens.len() {
                        let next_token = staged_tokens[j];
                        if next_token.token == TokenType::Intlit {
                            token.int_value = (token.int_value * 10) + next_token.int_value
                        } else {
                            i = j;
                            break;
                        }
                    }
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }

            tokens.push(token)
        }

        return tokens;
    }

    fn compile_primary_ast(&self, position: usize, tokens: &[Token]) -> ASTNode {
        let token = tokens[position];
        match token.token {
            TokenType::Intlit => ASTNode::new_leaf(AstnodeType::Intlit, token.int_value),
            tkn => panic!("syntax error on")
        }
    }

    fn compile_loop(&self, ptp: u32, position: usize, tokens: &[Token]) -> (ASTNode, usize) {
        let mut position_location = position;

        let mut left = self.compile_primary_ast(position_location, tokens);

        if tokens.len() <= position_location + 1 {
            return (left, position_location);
        }

        position_location += 1;

        let mut token = tokens[position_location];

        while token.token.precedence() > ptp {
            // Recursively call binexpr() with the
            // precedence of our token to build a sub-tree
            let (right, new_position_location) = self.compile_loop(token.token.precedence(), position_location + 1, tokens);

            position_location = new_position_location;

            // Join that sub-tree with ours. Convert the token
            // into an AST operation at the same time.
            left = ASTNode::new_node(AstnodeType::from(token.token), left, right, 0);
            token = tokens[new_position_location];
        }

        return (left, position_location);
    }

    fn compile_to_ast(&self, tokens: &[Token]) -> ASTNode {
        let (ast, location) = self.compile_loop(0, 0, tokens);
        ast
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Error(format!("{}", err))
    }
}