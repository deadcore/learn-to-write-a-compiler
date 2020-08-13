use std::iter::Peekable;

use log::{debug, info};

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpostamble, cgpreamble, cgprintint, cgsub};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::ast::AbstractSyntaxTreeNode;
use crate::scanner::{KeywordToken, Token, TokenIterator};
use crate::scanner::{Precedence, Scanner};

pub struct CodeGenerator<T: Iterator<Item=Token>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=Token>> CodeGenerator<T> {
    pub fn new(tokens: T) -> Self {
        CodeGenerator {
            inner: tokens.peekable()
        }
    }

    fn compile_int_keyword(&mut self) -> AbstractSyntaxTreeNode {
        let int_keyword = self.inner.next().expect("Expected int_keyword but received nothing");
        let identifier = self.inner.next().expect("Expected identifier but received nothing");
        let assignment_or_semi_colon = self.inner.next().expect("Expected assignment but received nothing");

        return match assignment_or_semi_colon {
            Token::SemiColon => {
                AbstractSyntaxTreeNode::new_construct(
                    AbstractSyntaxTreeConstructNodeType::Declaration,
                    AbstractSyntaxTreeNode::new_leaf_node(
                        AbstractSyntaxTreeLeafNodeType::from(identifier)
                    ),
                )
            }
            Token::Assignment => {
                let expression = self.compile_expression(0);

                AbstractSyntaxTreeNode::new_construct(
                    AbstractSyntaxTreeConstructNodeType::Declaration,
                    AbstractSyntaxTreeNode::new_interior(
                        AbstractSyntaxTreeExpressionNodeType::Assignment,
                        AbstractSyntaxTreeNode::new_leaf_node(
                            AbstractSyntaxTreeLeafNodeType::from(identifier)
                        ),
                        expression,
                    ),
                )
            }
            unhandled => panic!("Error - Expected [=] or [;] but got: [{:?}]", unhandled)
        };
    }

    fn compile_print_keyword(&mut self) -> AbstractSyntaxTreeNode {
        match self.inner.next() {
            Some(Token::Keyword(KeywordToken::Print)) => AbstractSyntaxTreeNode::new_construct(
                AbstractSyntaxTreeConstructNodeType::Print,
                self.compile_expression(0),
            ),
            unhandled => panic!("Error - Expected to compile u32 but instead got: {:?}", unhandled)
        }
    }

    fn compile_identifier(&mut self) -> AbstractSyntaxTreeNode {
        let identifier = self.inner.next().expect("Expected identifier but received nothing");
        let assignment = self.inner.next().expect("Expected assignment but received nothing");
        let expression = self.compile_expression(0);

        AbstractSyntaxTreeNode::new_interior(
            AbstractSyntaxTreeExpressionNodeType::Assignment,
            AbstractSyntaxTreeNode::new_leaf_node(
                AbstractSyntaxTreeLeafNodeType::from(identifier)
            ),
            expression,
        )
    }

    fn compile_expression(&mut self, ptp: u32) -> AbstractSyntaxTreeNode {
        let mut left = match self.inner.next() {
            Some(Token::U32(v)) => AbstractSyntaxTreeNode::new_leaf_node(AbstractSyntaxTreeLeafNodeType::U32(v)),
            Some(Token::Identifier(identifier)) => AbstractSyntaxTreeNode::new_leaf_node(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)),
            unhandled => panic!("Error - Expected to compile u32 but instead got: {:?}", unhandled)
        };

        while let Some(peeked_token) = self.inner.peek() {
            if peeked_token.precedence() > ptp {
                let token = self.inner.next().expect("Need another token to get");
                // Recursively call binexpr() with the
                // precedence of our token to build a sub-tree
                let right = self.compile_expression(token.precedence());

                // Join that sub-tree with ours. Convert the token
                // into an AST operation at the same time.
                left = AbstractSyntaxTreeNode::new_interior(AbstractSyntaxTreeExpressionNodeType::from(token), left, right);
            } else {
                return left;
            }
        }
        return left;
    }

    fn skip(&mut self) {
        let token = self.inner.next();
        debug!("Skipping token: {:?}", token)
    }
}

impl<T: Iterator<Item=Token>> Iterator for CodeGenerator<T> {
    type Item = AbstractSyntaxTreeNode;
    fn next(&mut self) -> Option<AbstractSyntaxTreeNode> {
        while let Some(token) = self.inner.peek() {
            debug!("Peeked a token: {:?}", token);

            match token {
                Token::Keyword(KeywordToken::Print) => return Some(self.compile_print_keyword()),
                Token::Keyword(KeywordToken::Int) => return Some(self.compile_int_keyword()),
                Token::Identifier(identifier) => return Some(self.compile_identifier()),
                Token::SemiColon => { self.skip() }
                Token::NewLine => { self.skip() }
                Token::Assignment => { self.skip() }
                unhandled => panic!("Unhandled token: [{:?}]", unhandled)
            };
        }
        return None;
    }
}