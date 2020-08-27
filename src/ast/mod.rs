use crate::scanner::{Token, KeywordToken};

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeNode {
    Construct(AbstractSyntaxTreeConstructNodeType, Box<AbstractSyntaxTreeNode>),
    Expression(AbstractSyntaxTreeExpressionNodeType, Box<AbstractSyntaxTreeNode>, Box<AbstractSyntaxTreeNode>),
    Leaf(AbstractSyntaxTreeLeafNodeType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbstractSyntaxTreeConstructNodeType {
    Print,
    Declaration,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbstractSyntaxTreeExpressionNodeType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assignment,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    NotEqual,
    Equality,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeLeafNodeType {
    U32(u32),
    SemiColon,
    Identifier(String),
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

impl From<KeywordToken> for AbstractSyntaxTreeConstructNodeType {
    fn from(token: KeywordToken) -> Self {
        match token {
            KeywordToken::Print => AbstractSyntaxTreeConstructNodeType::Print,
            unhandled => panic!("Unable to convert {:?} to an [AbstractSyntaxTreeUnaryNodeType]", unhandled)
        }
    }
}


impl From<Token> for AbstractSyntaxTreeExpressionNodeType {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => AbstractSyntaxTreeExpressionNodeType::Add,
            Token::Minus => AbstractSyntaxTreeExpressionNodeType::Subtract,
            Token::Star => AbstractSyntaxTreeExpressionNodeType::Multiply,
            Token::Slash => AbstractSyntaxTreeExpressionNodeType::Divide,
            Token::LessThan => AbstractSyntaxTreeExpressionNodeType::LessThan,
            Token::LessThanEqual => AbstractSyntaxTreeExpressionNodeType::LessThanEqual,
            Token::GreaterThan => AbstractSyntaxTreeExpressionNodeType::GreaterThan,
            Token::GreaterThanEqual => AbstractSyntaxTreeExpressionNodeType::GreaterThanEqual,
            Token::NotEqual => AbstractSyntaxTreeExpressionNodeType::NotEqual,
            Token::Equality => AbstractSyntaxTreeExpressionNodeType::Equality,
            unhandled => panic!("Unable to convert {:?} to an [AbstractSyntaxTreeInteriorNodeType]", unhandled)
        }
    }
}

impl AbstractSyntaxTreeNode {
    pub fn new_leaf_node(op: AbstractSyntaxTreeLeafNodeType) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Leaf(op)
    }

    pub fn new_interior(
        op: AbstractSyntaxTreeExpressionNodeType,
        left: AbstractSyntaxTreeNode,
        right: AbstractSyntaxTreeNode,
    ) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Expression(op, Box::new(left), Box::new(right))
    }

    pub fn new_construct(
        op: AbstractSyntaxTreeConstructNodeType,
        left: AbstractSyntaxTreeNode,
    ) -> AbstractSyntaxTreeNode {
        AbstractSyntaxTreeNode::Construct(op, Box::new(left))
    }
}