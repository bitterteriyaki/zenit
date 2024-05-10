mod tokens;
mod parser;
mod nodes;
mod generator;

use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{Error, Write};
use std::process::Command;

use tokens::Tokenizer;
use parser::Parser;
use generator::Generator;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    // Fow now, we will only accept two arguments.
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];
    let contents = read_to_string(filename)?;

    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let tree = parser.parse().unwrap();

    let generator = Generator::new(tree);
    let output = generator.generate();

    let mut file = File::create("out.asm")?;
    file.write_all(output.as_bytes())?;

    Command::new("nasm")
        .args(["-f", "elf64", "out.asm"])
        .output()?;
    Command::new("ld")
        .args(["-o", "out", "out.o"])
        .output()?;

    Ok(())
}
