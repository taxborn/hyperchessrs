use std::env;
use std::io::Read;
use std::fs::File;

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

fn parser(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 { 
            let instruct = match op {
                OpCode::IncrementX => Some(Instruction::IncrementX),
                OpCode::DecrementX => Some(Instruction::DecrementX),
                OpCode::IncrementY => Some(Instruction::IncrementY),
                OpCode::DecrementY => Some(Instruction::DecrementY),
                OpCode::IncrementZ => Some(Instruction::IncrementZ),
                OpCode::DecrementZ => Some(Instruction::DecrementZ),
                OpCode::IncrementW => Some(Instruction::IncrementW),
                OpCode::DecrementW => Some(Instruction::DecrementW),
                OpCode::Increment  => Some(Instruction::Increment),
                OpCode::Decrement  => Some(Instruction::Decrement),
                OpCode::Read       => Some(Instruction::Read),
                OpCode::Write      => Some(Instruction::Write),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;

                    None
                },

                OpCode::LoopEnd => panic!("Loop ending at #{} has no beginning", i),
            };

            match instruct {
                Some(instruct) => program.push(instruct),
                None => (),
            }
        } else {
            match op {
                OpCode::LoopBegin => loop_stack += 1,

                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parser(opcodes[loop_start+1 .. i].to_vec())));
                    }
                },

                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("Loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}

fn run(instructions: &Vec<Instruction>, tape: &mut Vec<Vec<Vec<Vec<u8>>>>, pointer_x: &mut usize, pointer_y: &mut usize, pointer_z: &mut usize, pointer_w: &mut usize) {
    for instruct in instructions {
        match instruct {
            Instruction::IncrementX => {
                *pointer_x += 1;
            },
            Instruction::DecrementX => *pointer_x -= 1,
            Instruction::IncrementY => *pointer_y += 1,
            Instruction::DecrementY => *pointer_y -= 1,
            Instruction::IncrementZ => *pointer_z += 1,
            Instruction::DecrementZ => *pointer_z -= 1,
            Instruction::IncrementW => *pointer_w += 1,
            Instruction::DecrementW => *pointer_w -= 1,
            Instruction::Increment  => tape[*pointer_x][*pointer_y][*pointer_z][*pointer_w] += 1,
            Instruction::Decrement  => tape[*pointer_x][*pointer_y][*pointer_z][*pointer_w] -= 1,
            Instruction::Write => print!("{}", tape[*pointer_x][*pointer_y][*pointer_z][*pointer_w] as char),
            Instruction::Read => (),
            Instruction::Loop(_) => (),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: hyperchessrs <name.4dc>");
    }

    let filename = &args[1];

    let mut file = File::open(filename).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source).expect("Failed to read to string.");

    let opcodes = lexer(source);
    let program = parser(opcodes);
    let mut tape: Vec<Vec<Vec<Vec<u8>>>> = vec![vec![vec![vec![0u8, 8]; 8]; 8]; 8];

    run(&program, &mut tape, &mut 0, &mut 0, &mut 0, &mut 0);

    // println!("{:#?}", tape);
}
