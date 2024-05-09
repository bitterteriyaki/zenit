use std::fmt::{Debug, Formatter, Result};
use std::process::exit;

#[derive(PartialEq, Clone)]
pub enum TokenType {
    Exit,
    IntegerLiteral,
    Semicolon,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenType::Exit => write!(f, "Exit"),
            TokenType::IntegerLiteral => write!(f, "IntegerLiteral"),
            TokenType::Semicolon => write!(f, "Semicolon"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    source: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer { source: input.to_string(), index: 0 }
    }

    fn peak(&self, offset: usize) -> Option<char> {
        if self.index + offset >= self.source.len() {
            return None;
        }

        let selected = self.source
            .chars()
            .nth(self.index + offset)
            .expect("Failed to peak character");

        Some(selected)
    }

    fn consume(&mut self) -> Option<char> {
        if self.index >= self.source.len() {
            return None;
        }

        let selected = self.source
            .chars()
            .nth(self.index)
            .expect("Failed to consume character");
        self.index += 1;

        Some(selected)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_token = String::new();

        while let Some(c) = self.consume() {
            if c.is_alphabetic() {
                current_token.push(c);

                while let Some(d) = self.peak(0) {
                    if !d.is_alphabetic() {
                        break;
                    }

                    let consumed = self.consume()
                        .expect("Failed to consume character");

                    current_token.push(consumed);
                }

                if current_token == "exit" {
                    tokens.push(Token { token_type: TokenType::Exit, value: None });
                    current_token.clear();
                    continue;
                } else {
                    eprintln!("Unexpected text token: {}", current_token);
                    exit(1);
                }
            } else if c.is_numeric() {
                current_token.push(c);

                while let Some(d) = self.peak(0) {
                    if !d.is_numeric() {
                        break;
                    }

                    let consumed = self.consume()
                        .expect("Failed to consume character");

                    current_token.push(consumed);
                }

                tokens.push(Token {
                    token_type: TokenType::IntegerLiteral,
                    value: Some(current_token.clone()),
                });
                current_token.clear();
            } else if c == ';' {
                tokens.push(Token { token_type: TokenType::Semicolon, value: None });
                current_token.clear();
                continue;
            } else if c.is_whitespace() {
                continue;
            } else {
                eprintln!("Unexpected unknown token: {}", c);
                exit(1);
            }
        }

        tokens
    }
}
