use crate::scanner::Token;

// AST node types
#[derive(Debug)]
pub enum AstnodeType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Intlit(u32),
}

// Abstract Syntax Tree structure
#[derive(Debug)]
pub struct ASTNode {
    pub op: AstnodeType,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl From<Token> for AstnodeType {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => AstnodeType::Add,
            Token::Minus => AstnodeType::Subtract,
            Token::Star => AstnodeType::Multiply,
            Token::Slash => AstnodeType::Divide,
            Token::U32(v) => AstnodeType::Intlit(v),
            _ => panic!("Error") // FIXME
        }
    }
}

impl ASTNode {
    pub fn new_leaf(op: AstnodeType) -> ASTNode {
        ASTNode::new(op, None, None)
    }

    pub fn new_node(
        op: AstnodeType,
        left: ASTNode,
        right: ASTNode,
    ) -> ASTNode {
        ASTNode::new(op, Some(Box::new(left)), Some(Box::new(right)))
    }

    pub fn new_unary(
        op: AstnodeType,
        left: Option<ASTNode>,
    ) -> ASTNode {
        ASTNode::new(op, left.map(|x| Box::new(x)), None)
    }

    fn new(
        op: AstnodeType,
        left: Option<Box<ASTNode>>,
        right: Option<Box<ASTNode>>,
    ) -> ASTNode {
        ASTNode {
            op,
            left,
            right,
        }
    }
}