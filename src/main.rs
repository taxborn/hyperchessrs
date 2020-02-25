use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[derive(Clone)]
enum OpCode {
    IncrementX,
    DecrementX,
    IncrementY,
    DecrementY,
    IncrementZ,
    DecrementZ,
    IncrementW,
    DecrementW,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}


#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    IncrementX,
    DecrementX,
    IncrementY,
    DecrementY,
    IncrementZ,
    DecrementZ,
    IncrementW,
    DecrementW,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn lexer (source: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '+'       => Some(OpCode::Increment),
            '-'       => Some(OpCode::Decrement),
            '>'       => Some(OpCode::IncrementX),
            '<'       => Some(OpCode::DecrementX),
            '^'       => Some(OpCode::IncrementY),
            'v' | 'V' => Some(OpCode::DecrementY),
            '*'       => Some(OpCode::IncrementZ),
            'o' | 'O' => Some(OpCode::DecrementZ),
            '@'       => Some(OpCode::IncrementW),
            '?'       => Some(OpCode::DecrementW),
            '.'       => Some(OpCode::Write),
            ','       => Some(OpCode::Read),
            '['       => Some(OpCode::LoopBegin),
            ']'       => Some(OpCode::LoopEnd),
            _         => None,
        };

        match op {
            Some(op) => operations.push(op),
            None     => (),
        }
    }

    operations
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let filename = &args[1];

            let f = BufReader::new(File::open(filename).expect("Open failed"));

            for line in f.lines() {
                for c in line.expect("lines failed").chars() {
                    interpret(c);
                }
            }
        },
        _ => {
            println!("Please specify ONE file to compile.");
        }
    }
}
