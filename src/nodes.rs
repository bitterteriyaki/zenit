use crate::tokens::Token;

pub struct NodeExpr {
    pub integer: Token,
}

pub struct NodeExit {
    pub expr: NodeExpr,
}
