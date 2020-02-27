use std::io::Read;

use crate::instructions::Instruction;

pub fn run(instructions: &Vec<Instruction>, tape: &mut Vec<Vec<Vec<Vec<u8>>>>, pointer_x: &mut usize, pointer_y: &mut usize, pointer_z: &mut usize, pointer_w: &mut usize) {
    for instruct in instructions {
        match instruct {
            Instruction::IncrementX => *pointer_x += 1,
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
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];

                std::io::stdin().read_exact(&mut input).expect("Failed to read data.");

                tape[*pointer_x][*pointer_y][*pointer_z][*pointer_w] = input[0];
            },
            Instruction::Loop(instructions) => {
                while tape[*pointer_x][*pointer_y][*pointer_z][*pointer_w] != 0 {
                    run(&instructions, tape, &mut 0, &mut 0, &mut 0, &mut 0);
                }
            },
        }
    }
}