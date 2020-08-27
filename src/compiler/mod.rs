use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Chars;

use log::debug;

use crate::asm::{cgadd, cgcomment, cgdiv, cgglobsym, cgload, cgloadglob, cgmul, cgpostamble, cgpreamble, cgprintint, cgstorglob, cgsub, cglessthan};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::ast::AbstractSyntaxTreeNode;
use crate::compiler::code_generator::CodeGenerator;
use crate::scanner::{TokenIterator, Token};
use std::path::Path;

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

    pub fn compile<P: AsRef<Path>>(&self, path: P) -> core::result::Result<(), Box<dyn std::error::Error>> {
        let file = path.as_ref();
        debug!("Compiling file: {:?}", file);

        let file_name = file.file_stem().unwrap().to_os_string().into_string().unwrap();

        let content = fs::read_to_string(file).unwrap(); // FIXME
        let chars: Chars = content.chars();

        let tokens = TokenIterator::new_iterator(chars).filter(|x| *x != Token::Space);

        let code_generator = CodeGenerator::new(tokens);

        let mut out = File::create(format!("{}.s", file_name))?;
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

        use std::process::Command;

        let mut cc = Command::new("cc");
        cc.arg("-o");
        cc.arg(&file_name);
        cc.arg(format!("{}.s", file_name));

        debug!("command: {:?}", cc);

        let result = cc.spawn().unwrap();

        // .arg("-l")
            // .arg("-a");
        // .spawn()
        // .expect("ls command failed to start");

        debug!("result: {:?}", result);


        Ok(())
    }

    fn interpret_ast_to_asm<W: Write>(&self, w: &mut W, registers: &mut Registers, ast: AbstractSyntaxTreeNode) -> core::result::Result<Option<RegisterIndex>, Box<dyn std::error::Error>> {
        debug!("Interpreting abstract syntax tree: {:?}", ast);
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
            AbstractSyntaxTreeNode::Expression(AbstractSyntaxTreeExpressionNodeType::LessThan, left, right) => {
                Ok(
                    Some(
                        cglessthan(
                            self.interpret_ast_to_asm(w, registers, *left)?.expect("Expected a value to be placed in a register"),
                            self.interpret_ast_to_asm(w, registers, *right)?.expect("Expected a value to be placed in a register"),
                            registers,
                            w,
                        )?
                    )
                )
            }
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