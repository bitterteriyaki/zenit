pub mod node {
    use crate::tokens::Token;

    pub struct NodeExpr {
        pub integer_literal: Token,
    }

    pub struct NodeExit {
        pub expr: NodeExpr,
    }
}

use std::process::exit;

use crate::tokens::{Token, TokenType};
use node::{NodeExit, NodeExpr};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn peak(&self, offset: usize) -> Option<&Token> {
        if self.index + offset >= self.tokens.len() {
            return None;
        }

        let selected = self.tokens
            .get(self.index + offset)
            .expect("Failed to peak character");

        Some(selected)
    }

    fn consume(&mut self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }

        let selected = self.tokens
            .get(self.index)
            .expect("Failed to consume character");
        self.index += 1;

        Some(selected)
    }

    fn parse_expr(&mut self) -> Option<NodeExpr> {
        if let Some(t) = self.peak(0) {
            if t.token_type == TokenType::IntegerLiteral {
                return Some(NodeExpr { integer_literal: self.consume().unwrap().clone() });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_code = None;

        while let Some(t) = self.consume() {
            if t.token_type == TokenType::Exit {
                let expr = self.parse_expr()
                    .expect("Failed to parse expression");
                let next_token = self.consume()
                    .expect("Failed to consume token after exit statement");

                exit_code = Some(NodeExit { expr });

                if next_token.token_type != TokenType::Semicolon {
                    eprintln!("Expected semicolon after exit statement");
                    exit(1);
                }
            }
        }

        exit_code
    }
}
