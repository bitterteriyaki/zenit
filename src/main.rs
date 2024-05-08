use std::env::args;
use std::io::Write;
use std::process::exit;
use std::fs::{read_to_string, File};
use std::fmt::{Debug, Formatter, Result};
use std::process::Command;

enum TokenType {
    Return,
    IntegerLiteral,
    Semicolon,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenType::Return => write!(f, "Return"),
            TokenType::IntegerLiteral => write!(f, "IntegerLiteral"),
            TokenType::Semicolon => write!(f, "Semicolon"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = 0;

    while current < input.len() {
        let c = input.chars().nth(current).unwrap();

        match c {
            '0'..='9' => {
                let mut value = String::new();
                let mut i = current + 1;

                value.push(c);

                while i < input.len() {
                    let c = input.chars().nth(i).unwrap();

                    if !c.is_ascii_digit() {
                        break;
                    }

                    value.push(c);
                    i += 1;
                }

                tokens.push(Token {
                    token_type: TokenType::IntegerLiteral,
                    value: Some(value),
                });
                
                current = i;
            }
            ' ' => {
                current += 1;
            }
            '\n' => {
                current += 1;
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: None,
                });
            
                current += 1;
            }
            _ => {
                let mut value = String::new();
                let mut i = current + 1;
                
                value.push(c);
                
                while i < input.len() {
                    let c = input.chars().nth(i).unwrap();
                    
                    if !c.is_ascii_alphabetic() {
                        break;
                    }

                    value.push(c);
                    i += 1;
                }

                match value.as_str() {
                    "return" => {
                        tokens.push(Token {
                            token_type: TokenType::Return,
                            value: None,
                        });
                    }
                    _ => {
                        eprintln!("Unexpected token: {}", value);
                        exit(1);
                    }
                }

                current = i;
            }
        }
    }

    tokens
}

fn tokens_to_asm(tokens: Vec<Token>) -> String {
    let mut asm = String::new();
    asm.push_str("global _start\n\n");
    asm.push_str("_start:\n");

    let mut current = 0;

    while current < tokens.len() {
        let token = &tokens[current];

        match token.token_type {
            TokenType::Return => {
                if current + 1 >= tokens.len() {
                    eprintln!("Unexpected end of input");
                    exit(1);
                }

                match &tokens[current + 1].token_type {
                    TokenType::IntegerLiteral => {
                        if current + 2 >= tokens.len() {
                            eprintln!("Unexpected end of input");
                            exit(1);
                        }

                        match &tokens[current + 2].token_type {
                            TokenType::Semicolon => {
                                let value = tokens[current + 1]
                                    .value
                                    .as_ref()
                                    .expect("Expected integer literal");

                                asm.push_str("    mov rax, 60\n");
                                asm.push_str(format!("    mov rdi, {}\n", value).as_str());
                                asm.push_str("    syscall\n");
                                current += 3;
                            }
                            _ => {
                                eprintln!("Unexpected token: {:?}", tokens[current + 2]);
                                exit(1);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unexpected token: {:?}", tokens[current + 1]);
                        exit(1);
                    }
                }
            }
            _ => {
                eprintln!("Unexpected token: {:?}", token);
                exit(1);
            }
        }
    }

    asm
}

fn main() {
    let argv: Vec<String> = args().collect();

    if argv.len() < 2 {
        eprintln!("Usage: {} <filename>", argv[0]);
        exit(1);
    }

    let path = &argv[1];
    let contents = read_to_string(path)
        .expect("Something went wrong reading the file");

    let tokens = tokenize(&contents);
    let asm = tokens_to_asm(tokens);

    let mut file = File::create("out.asm")
        .expect("Failed to create output file");
    file.write_all(asm.as_bytes())
        .expect("Failed to write to output file");

    Command::new("nasm")
        .args(["-felf64", "out.asm"])
        .output()
        .expect("Failed to compile assembly");
    Command::new("ld")
        .args(["out.o", "-o", "out"])
        .output()
        .expect("Failed to link object file");
}
