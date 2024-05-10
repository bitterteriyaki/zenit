use std::collections::LinkedList;

use crate::tokens::{Token, TokenKind};
use crate::nodes::{NodeExit, NodeExpr};

pub struct Parser {
    tokens: LinkedList<Token>,
}

impl Parser {
    pub fn new(tokens: LinkedList<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node = None;

        while let Some(token) = self.tokens.pop_front() {
            match token.kind {
                TokenKind::Exit => {
                    let expr = self.parse_expr()?;
                    let next_token = self.tokens.pop_front()?;

                    if next_token.kind != TokenKind::Semicolon {
                        return None;
                    }

                    exit_node = Some(NodeExit { expr });
                },
                _ => return None,
            }
        }

        exit_node
    }

    fn parse_expr(&mut self) -> Option<NodeExpr> {
        let token = self.tokens.front();

        if token.is_none() {
            return None;
        }

        match token?.kind {
            TokenKind::Integer => {
                let integer = self.tokens.pop_front()?;
                Some(NodeExpr { integer })
            }
            _ => None,
        }
    }
}
