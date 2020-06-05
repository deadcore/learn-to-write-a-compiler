pub mod code_generator;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Chars;

use clap::ArgMatches;
use log::{debug, info};

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpostamble, cgpreamble, cgprintint, cgsub};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::scanner::{Precedence, Scanner, Token};
use crate::ast::AbstractSyntaxTreeNode;
use crate::compiler::code_generator::CodeGenerator;

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
    pub fn from_arg_matches() -> Compiler {
        Compiler::new(
            Scanner::from_arg_matches()
        )
    }

    fn new(scanner: Scanner) -> Compiler {
        Compiler { scanner }
    }

    pub fn compile(&self, filename: &str) -> Result<()> {
        debug!("Compiling file: {}", filename);

        let content = fs::read_to_string(filename).unwrap(); // FIXME
        let chars: Chars = content.chars();

        let tokens = self.scanner.new_iterator(chars).filter(|x| *x != Token::Space);

        let code_generator = CodeGenerator::new(tokens);

        let mut out = File::create("out.s")?;
        let mut registers = Registers::new();

        cgpreamble(out.by_ref());

        for code in code_generator {
            debug!("Abstract Syntax Tree: {:#?}", code);
            self.interpret_ast_to_asm(out.by_ref(), &mut registers, code);
            registers.free_all();
        }

        cgpostamble(out.by_ref());

        Ok(())
    }

    fn interpret_ast_to_asm(&self, w: &mut dyn Write, registers: &mut Registers, ast: AbstractSyntaxTreeNode) -> RegisterIndex {
        return match ast {
            AbstractSyntaxTreeNode::Interior(AbstractSyntaxTreeInteriorNodeType::Add, left, right) => cgadd(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            AbstractSyntaxTreeNode::Interior(AbstractSyntaxTreeInteriorNodeType::Subtract, left, right) => cgsub(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            AbstractSyntaxTreeNode::Interior(AbstractSyntaxTreeInteriorNodeType::Multiply, left, right) => cgmul(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            AbstractSyntaxTreeNode::Interior(AbstractSyntaxTreeInteriorNodeType::Divide, left, right) => cgdiv(self.interpret_ast_to_asm(w, registers, *left), self.interpret_ast_to_asm(w, registers, *right), registers, w),
            AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::U32(i)) => cgload(i, registers, w),
            AbstractSyntaxTreeNode::Unary(AbstractSyntaxTreeUnaryNodeType::Print, left) => cgprintint(self.interpret_ast_to_asm(w, registers, *left), w),
            unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
        };
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Error(format!("{}", err))
    }
}