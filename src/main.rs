use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let filename = &args[1];

            let f = BufReader::new(File::open(filename).expect("Open failed"));

            for line in f.lines() {
                for c in line.expect("lines failed").chars(){
                    println!("Char: {}", c);
                }
            }
        },
        _ => {
            println!("Please specify ONE file to compile.");
        }
    }
}
