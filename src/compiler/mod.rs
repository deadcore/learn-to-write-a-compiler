pub mod code_generator;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Chars;

use clap::ArgMatches;
use log::{debug, info};

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpostamble, cgpreamble, cgprintint, cgsub, cgcomment, cgglobsym, cgstorglob, cgloadglob};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::scanner::{Precedence, Scanner, Token};
use crate::ast::AbstractSyntaxTreeNode;
use crate::compiler::code_generator::CodeGenerator;
use std::borrow::Borrow;

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
        cgcomment(out.by_ref(), "Starting users code");

        for code in code_generator {
            debug!("Abstract Syntax Tree: {:#?}", code);
            self.interpret_ast_to_asm(out.by_ref(), &mut registers, code);
            registers.free_all();
        }

        cgcomment(out.by_ref(), "Ending users code");
        cgpostamble(out.by_ref());

        Ok(())
    }

    fn interpret_ast_to_asm<W: Write>(&self, w: &mut W, registers: &mut Registers, ast: AbstractSyntaxTreeNode) -> Option<RegisterIndex> {
        return match ast {
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Add, left, right) =>
                Some(
                    cgadd(
                        self.interpret_ast_to_asm(w, registers, *left).expect("Expected a value to be placed in a register"),
                        self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                        registers,
                        w,
                    )
                ),
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Subtract, left, right) =>
                Some(
                    cgsub(
                        self.interpret_ast_to_asm(w, registers, *left).expect("Expected a value to be placed in a register"),
                        self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                        registers,
                        w,
                    )
                ),
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Multiply, left, right) => {
                Some(
                    cgmul(
                        self.interpret_ast_to_asm(w, registers, *left).expect("Expected a value to be placed in a register"),
                        self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                        registers,
                        w,
                    )
                )
            }
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Divide, left, right) =>
                Some(
                    cgdiv(
                        self.interpret_ast_to_asm(w, registers, *left).expect("Expected a value to be placed in a register"),
                        self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                        registers,
                        w,
                    )
                ),
            AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::U32(i)) =>
                Some(
                    cgload(
                        i,
                        registers,
                        w,
                    )
                ),
            AbstractSyntaxTreeNode::Construct(AbstractSyntaxTreeConstructNodeType::Print, left) => {
                cgprintint(
                    self.interpret_ast_to_asm(w, registers, *left).expect("Expected a value to be placed in a register"),
                    w,
                );
                None
            }
            AbstractSyntaxTreeNode::Construct(AbstractSyntaxTreeConstructNodeType::Declaration, left) => {
                self.interpret_ast_to_asm(w, registers, *left);
                None
                //     None
                //     // match *left {
                //     //     AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeInteriorNodeType::Assignment, AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)), right) => {
                //     //         cgglobsym(&identifier, w);
                //     //         cgstorglob(
                //     //             &identifier,
                //     //             self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                //     //             w,
                //     //         )
                //     //     }
                //     //     unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
                //     // }
            }
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Assignment, left, right) => {
                match *left {
                    AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)) => {
                        cgglobsym(&identifier, w);
                        let index = cgstorglob(
                            &identifier,
                            self.interpret_ast_to_asm(w, registers, *right).expect("Expected a value to be placed in a register"),
                            w,
                        );

                        Some(index)
                    }
                    unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
                }
            }
            AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)) => {
                Some(cgloadglob(&identifier, registers, w))
            }
            unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
        };
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Error(format!("{}", err))
    }
}