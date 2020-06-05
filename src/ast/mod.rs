use crate::scanner::{Token, KeywordToken};

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeNode {
    Unary(AbstractSyntaxTreeUnaryNodeType, Box<AbstractSyntaxTreeNode>),
    Interior(AbstractSyntaxTreeInteriorNodeType, Box<AbstractSyntaxTreeNode>, Box<AbstractSyntaxTreeNode>),
    Leaf(AbstractSyntaxTreeLeafNodeType),
    Construct(AbstractSyntaxTreeConstructNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeConstructNode {
    StatementSequence(Vec<AbstractSyntaxTreeNode>)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbstractSyntaxTreeInteriorNodeType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assignment,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeLeafNodeType {
    U32(u32),
    SemiColon,
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbstractSyntaxTreeUnaryNodeType {
    Print
}

impl From<Token> for AbstractSyntaxTreeLeafNodeType {
    fn from(token: Token) -> Self {
        match token {
            Token::U32(v) => AbstractSyntaxTreeLeafNodeType::U32(v),
            Token::Identifier(v) => AbstractSyntaxTreeLeafNodeType::Identifier(v),
            unhandled => panic!("Unable to convert {:?} to an [AbstractSyntaxTreeLeafNodeType]", unhandled)
        }
    }
}

impl From<KeywordToken> for AbstractSyntaxTreeUnaryNodeType {
    fn from(token: KeywordToken) -> Self {
        match token {
            KeywordToken::Print => AbstractSyntaxTreeUnaryNodeType::Print,
            unhandled => panic!("Unable to convert {:?} to an [AbstractSyntaxTreeUnaryNodeType]", unhandled)
        }
    }
}


impl From<Token> for AbstractSyntaxTreeInteriorNodeType {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => AbstractSyntaxTreeInteriorNodeType::Add,
            Token::Minus => AbstractSyntaxTreeInteriorNodeType::Subtract,
            Token::Star => AbstractSyntaxTreeInteriorNodeType::Multiply,
            Token::Slash => AbstractSyntaxTreeInteriorNodeType::Divide,
            unhandled => panic!("Unable to convert {:?} to an [AbstractSyntaxTreeInteriorNodeType]", unhandled)
        }
    }
}

impl AbstractSyntaxTreeNode {
    pub fn new_leaf_node(op: AbstractSyntaxTreeLeafNodeType) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Leaf(op)
    }

    pub fn new_interior_node(
        op: AbstractSyntaxTreeInteriorNodeType,
        left: AbstractSyntaxTreeNode,
        right: AbstractSyntaxTreeNode,
    ) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Interior(op, Box::new(left), Box::new(right))
    }

    pub fn new_unary_node(
        op: AbstractSyntaxTreeUnaryNodeType,
        left: AbstractSyntaxTreeNode,
    ) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Unary(op, Box::new(left))
    }
}