use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn interpret(command: char) {
    match command {
        '+' => {
            println!("We are incrementing the current cell value.");
        },
        '-' => {
            println!("We are decrementing the current cell value.");
        },
        '>' => {
            println!("We are increasing the pointer along the X axis.");
        },
        '<' => {
            println!("We are decreasing the pointer along the X axis.");
        },
        '^' => {
            println!("We are increasing the pointer along the Y axis.");
        },
        'v' | 'V' => {
            println!("We are decreasing the pointer along the Y axis.");
        },
        '*' => {
            println!("We are increasing the pointer along the Z axis.");
        },
        'o' | 'O' => {
            println!("We are decreasing the pointer along the Z axis.");
        },
        '@' => {
            println!("We are increasing the pointer along the W axis.");
        },
        '?' => {
            println!("We are decreasing the pointer along the W axis.");
        },
        '.' => {
            println!("We are outputting current cell value here.");
        },
        ',' => {
            println!("We are inputting a character here.");
        },
        '[' => {
            println!("Jump to matching brace if pointer is zero.");
        },
        ']' => {
            println!("Jump to matching brace if pointer is nonzero");
        },
        _ => {
            println!("How did you get here?");
        }
    }
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
