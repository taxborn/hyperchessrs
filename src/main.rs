use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("chess.4dc").expect("Open failed"));

    for line in f.lines() {
        for c in line.expect("lines failed").chars(){
            println!("Char: {}", c);
        }
    }
}
