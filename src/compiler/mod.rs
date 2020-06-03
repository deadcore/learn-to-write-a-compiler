use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Chars;

use clap::ArgMatches;

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpostamble, cgpreamble, cgprintint, cgsub};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::scanner::{Precedence, Scanner, Token};

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
        let content = fs::read_to_string(filename).unwrap(); // FIXME
        let chars: Chars = content.chars();

        let tokens: Vec<Token> = self.scanner.new_iterator(chars).collect();

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
            (_, AstnodeType::Intlit(i), _) => cgload(i, registers, w),
            _ => panic!("Unhandled")
        };
    }

    fn interpret_ast(&self, ast: ASTNode) -> u32 {
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
            AstnodeType::Intlit(i) => i,
        };
    }

    fn compile_primary_ast(&self, position: usize, tokens: &[Token]) -> ASTNode {
        let token = tokens[position];
        match token {
            Token::U32(v) => ASTNode::new_leaf(AstnodeType::Intlit(v)),
            tkn => panic!("syntax error with token: {:?}", tkn)
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

        while token.precedence() > ptp {
            // Recursively call binexpr() with the
            // precedence of our token to build a sub-tree
            let (right, new_position_location) = self.compile_loop(token.precedence(), position_location + 1, tokens);

            position_location = new_position_location;

            // Join that sub-tree with ours. Convert the token
            // into an AST operation at the same time.
            left = ASTNode::new_node(AstnodeType::from(token), left, right);
            token = tokens[new_position_location];
        }

        return (left, position_location);
    }

    fn compile_to_ast(&self, tokens: &[Token]) -> ASTNode {
        let (ast, _) = self.compile_loop(0, 0, tokens);
        ast
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Error(format!("{}", err))
    }
}