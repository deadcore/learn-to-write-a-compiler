use std::iter::Peekable;

use log::{debug, info};

use crate::asm::{cgadd, cgdiv, cgload, cgmul, cgpostamble, cgpreamble, cgprintint, cgsub};
use crate::asm::registers::{RegisterIndex, Registers};
use crate::ast::*;
use crate::ast::AbstractSyntaxTreeNode;
use crate::scanner::{KeywordToken, Token, TokenIterator};
use crate::scanner::{Precedence, Scanner};
use crate::ast::AbstractSyntaxTreeConstructNode::StatementSequence;

pub struct CodeGenerator<T: Iterator<Item=Token>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=Token>> CodeGenerator<T> {
    pub fn new(tokens: T) -> Self {
        CodeGenerator {
            inner: tokens.peekable()
        }
    }

    fn compile_int_keyword(&mut self, ptp: u32) -> Option<AbstractSyntaxTreeNode> {
        let int_keyword = self.inner.next()?;
        let identifier = self.inner.next()?;
        let assignment = self.inner.next()?;

        return Some(
            AbstractSyntaxTreeNode::new_interior_node(
                AbstractSyntaxTreeInteriorNodeType::Assignment,
                AbstractSyntaxTreeNode::new_leaf_node(AbstractSyntaxTreeLeafNodeType::from(identifier)),
                self.compile_expression(0),
            )
        );
    }

    fn compile_print_keyword(&mut self) -> AbstractSyntaxTreeNode {
        match self.inner.next() {
            Some(Token::Keyword(KeywordToken::Print)) => AbstractSyntaxTreeNode::new_unary_node(
                AbstractSyntaxTreeUnaryNodeType::Print,
                self.compile_expression(0),
            ),
            unhandled => panic!("Error - Expected to compile u32 but instead got: {:?}", unhandled)
        }
    }

    fn compile_identifier(&mut self) -> AbstractSyntaxTreeNode {
        match self.inner.next() {
            Some(Token::Identifier(identifier)) => AbstractSyntaxTreeNode::new_leaf_node(AbstractSyntaxTreeLeafNodeType::Identifier(identifier)),
            unhandled => panic!("Error - Expected to compile u32 but instead got: {:?}", unhandled)
        }
    }

    fn compile_expression(&mut self, ptp: u32) -> AbstractSyntaxTreeNode {
        let mut left = match self.inner.next() {
            Some(Token::U32(v)) => AbstractSyntaxTreeNode::new_leaf_node(AbstractSyntaxTreeLeafNodeType::U32(v)),
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
                left = AbstractSyntaxTreeNode::new_interior_node(AbstractSyntaxTreeInteriorNodeType::from(token), left, right);
            } else {
                return left;
            }
        }
        return left;
    }
}

impl<T: Iterator<Item=Token>> Iterator for CodeGenerator<T> {
    type Item = AbstractSyntaxTreeNode;
    fn next(&mut self) -> Option<AbstractSyntaxTreeNode> {
        while let Some(token) = self.inner.peek() {
            debug!("Peeked a token: {:?}", token);

            match token {
                Token::Keyword(KeywordToken::Print) => return Some(self.compile_print_keyword()),
                Token::Identifier(identifier) => return Some(self.compile_identifier()),
                Token::SemiColon => { self.inner.next(); }
                Token::NewLine => { self.inner.next(); }
                unhandled => panic!("Unhandled token: [{:?}]", unhandled)
            };
        }
        return None;
    }
}