pub mod tokens;
pub mod parser;
pub mod generator;

use std::env::args;
use std::io::Write;
use std::process::exit;
use std::fs::{read_to_string, File};
use std::process::Command;
use tokens::Tokenizer;
use generator::Generator;
use parser::Parser;

fn main() {
    let argv: Vec<String> = args().collect();

    if argv.len() < 2 {
        eprintln!("Usage: {} <filename>", argv[0]);
        exit(1);
    }

    let path = &argv[1];
    let contents = read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let tree = parser
        .parse()
        .unwrap();

    let generator = Generator::new(tree);

    let mut file = File::create("out.asm")
        .expect("Failed to create output file");
    file.write_all(generator.generate().as_bytes())
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
