use std::collections::LinkedList;
use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq)]
pub enum TokenKind {
    Exit,
    Integer,
    Semicolon,
}

impl Debug for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenKind::Exit => write!(f, "Exit"),
            TokenKind::Integer => write!(f, "Integer"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
        }
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

pub struct Tokenizer {
    source: LinkedList<char>,
}

impl Tokenizer {
    pub fn new(contents: &str) -> Tokenizer {
        Tokenizer { source: contents.chars().collect() }
    }

    pub fn tokenize(&mut self) -> LinkedList<Token> {
        let mut tokens: LinkedList<Token> = LinkedList::new();
        let mut buffer = String::new();
        // Keep the original source in case we need it later.
        let mut source = self.source.clone();
        

        while let Some(ch) = source.pop_front() {
            match ch {
                t if t.is_alphabetic() => {
                    buffer.push(t);

                    // Consume all the characters that are alphabetic.
                    while let Some(&c) = source.front() {
                        if !c.is_alphabetic() {
                            break;
                        }

                        buffer.push(c);
                        source.pop_front();
                    }

                    match buffer.as_str() {
                        "exit" => tokens.push_back(Token { kind: TokenKind::Exit, value: None }),
                        _ => panic!("Unknown token: {}", buffer),
                    }

                    buffer.clear();
                },
                t if t.is_numeric() => {
                    buffer.push(t);

                    // Consume all the characters that are numeric.
                    while let Some(&c) = source.front() {
                        if !c.is_numeric() {
                            break;
                        }

                        buffer.push(c);
                        source.pop_front();
                    }

                    tokens.push_back(Token { kind: TokenKind::Integer, value: Some(buffer.clone()) });
                    buffer.clear();
                },
                ';' => tokens.push_back(Token { kind: TokenKind::Semicolon, value: None }),
                ' ' | '\n' => continue,
                _ => panic!("Unknown character: {}", ch),
            }
        }

        tokens
    }
}
