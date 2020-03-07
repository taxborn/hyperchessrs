mod instructions;
mod lexer;
mod parser;
mod runner;

use std::env;
use std::fs::File;
use std::io::Read;

use crate::lexer::lex;
use crate::parser::parse;
use crate::runner::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: hyperchessrs <name.4dc>");
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Failed to read to string.");

    let length: usize = 8;

    let opcodes = lex(source);
    let program = parse(opcodes);
    let mut tape: Vec<Vec<Vec<Vec<u8>>>> =
        vec![vec![vec![vec![0u8; length]; length]; length]; length];

    let mut pointers: [usize; 4] = [0; 4];
    run(&program, &mut tape, &mut pointers);
}
