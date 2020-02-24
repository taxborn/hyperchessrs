use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn interpret(command: char) {
    println!("Char: {}", command);
}

fn main() {
    let valid_characters = ['<', '>', '^', 'v', 'V', '*', 'o', 'O', '?', '@', '+', '-', '.', ',', '[', ']'];
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let filename = &args[1];

            let f = BufReader::new(File::open(filename).expect("Open failed"));

            for line in f.lines() {
                for c in line.expect("lines failed").chars() {
                    if valid_characters.contains(&c) {
                        interpret(c);
                    }
                }
            }
        },
        _ => {
            println!("Please specify ONE file to compile.");
        }
    }
}
