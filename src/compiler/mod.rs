use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Chars;

use log::debug;

use crate::asm::{cgadd, cgcomment, cgdiv, cgglobsym, cgload, cgloadglob, cgmul, cgpostamble, cgpreamble, cgprintint, cgstorglob, cgsub};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::ast::AbstractSyntaxTreeNode;
use crate::compiler::code_generator::CodeGenerator;
use crate::scanner::{TokenIterator, Token};

pub mod code_generator;

/// An error as returned by a `Handler` method.
#[derive(Debug)]
pub enum Error {
    Error(String),
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, filename: &str) -> core::result::Result<(), Box<dyn std::error::Error>> {
        debug!("Compiling file: {}", filename);

        let content = fs::read_to_string(filename).unwrap(); // FIXME
        let chars: Chars = content.chars();

        let tokens = TokenIterator::new_iterator(chars).filter(|x| *x != Token::Space);

        let code_generator = CodeGenerator::new(tokens);

        let mut out = File::create(format!("{}.s", filename))?;
        let mut registers = Registers::new();

        cgpreamble(out.by_ref())?;
        cgcomment(out.by_ref(), "Starting users code")?;

        for code in code_generator {
            debug!("Abstract Syntax Tree: {:#?}", code);
            self.interpret_ast_to_asm(out.by_ref(), &mut registers, code)?;
            registers.free_all();
        }

        cgcomment(out.by_ref(), "Ending users code")?;
        cgpostamble(out.by_ref())?;

        Ok(())
    }

    fn interpret_ast_to_asm<W: Write>(&self, w: &mut W, registers: &mut Registers, ast: AbstractSyntaxTreeNode) -> core::result::Result<Option<RegisterIndex>, Box<dyn std::error::Error>> {
        return match ast {
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Add, left, right) =>
                Ok(
                    Some(
                        cgadd(
                            self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            registers,
                            w,
                        )?
                    )
                ),
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Subtract, left, right) =>
                Ok(
                    Some(
                        cgsub(
                            self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            registers,
                            w,
                        )?
                    )
                ),
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Multiply, left, right) => {
                Ok(
                    Some(
                        cgmul(
                            self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            registers,
                            w,
                        )?
                    )
                )
            }
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Divide, left, right) =>
                Ok(
                    Some(
                        cgdiv(
                            self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            registers,
                            w,
                        )?
                    )
                ),
            AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::U32(i)) =>
                Ok(
                    Some(
                        cgload(
                            i,
                            registers,
                            w,
                        )?
                    )
                ),
            AbstractSyntaxTreeNode::Construct(AbstractSyntaxTreeConstructNodeType::Print, left) => {
                cgprintint(
                    self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                    w,
                )?;
                Ok(None)
            }
            AbstractSyntaxTreeNode::Construct(AbstractSyntaxTreeConstructNodeType::Declaration, left) => {
                self.interpret_ast_to_asm(w, registers, *left)
            }
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::Assignment, left, right) => {
                match *left {
                    AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)) => {
                        cgglobsym(&identifier, w)?;

                        let index = cgstorglob(
                            &identifier,
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            w,
                        );
                        index.map(Some)
                    }
                    unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
                }
            }
            AbstractSyntaxTreeNode::Leaf(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)) => {
                cgloadglob(&identifier, registers, w).map(Some)
            }
            unhandled => panic!("Unhandled abstract syntax tree element: {:?}", unhandled),
        };
    }
}