use crate::scanner::Token;

// AST node types
#[derive(Debug)]
pub enum AstnodeType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Intlit(i32),
}

// Abstract Syntax Tree structure
#[derive(Debug)]
pub struct ASTNode {
    pub op: AstnodeType,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl From<TokenType> for AstnodeType {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => AstnodeType::Add,
            TokenType::Minus => AstnodeType::Subtract,
            TokenType::Star => AstnodeType::Multiply,
            TokenType::Slash => AstnodeType::Divide,
            TokenType::Intlit => AstnodeType::Intlit
        }
    }
}

impl ASTNode {
    pub fn new_leaf(op: AstnodeType, int_value: i32) -> ASTNode {
        ASTNode::new(op, None, None, int_value)
    }

    pub fn new_node(
        op: AstnodeType,
        left: ASTNode,
        right: ASTNode,
        int_value: i32,
    ) -> ASTNode {
        ASTNode::new(op,
                     Some(Box::new(left)),
                     Some(Box::new(right)),
                     int_value)
    }

    pub fn new_unary(
        op: AstnodeType,
        int_value: i32,
        left: Option<ASTNode>,
    ) -> ASTNode {
        ASTNode::new(op,
                     left.map(|x| Box::new(x)),
                     None,
                     int_value)
    }

    fn new(
        op: AstnodeType,
        left: Option<Box<ASTNode>>,
        right: Option<Box<ASTNode>>,
        int_value: i32,
    ) -> ASTNode {
        ASTNode {
            op,
            left,
            right,
            int_value,
        }
    }
}