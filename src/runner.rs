use std::io::Read;

use crate::instructions::Instruction;

pub fn run(instructions: &Vec<Instruction>, tape: &mut Vec<Vec<Vec<Vec<u8>>>>, pointers: &mut [usize]) {
    for instruct in instructions {
        match instruct {
            Instruction::IncrementX => pointers[0] += 1,
            Instruction::DecrementX => pointers[0] -= 1,
            Instruction::IncrementY => pointers[1] += 1,
            Instruction::DecrementY => pointers[1] -= 1,
            Instruction::IncrementZ => pointers[2] += 1,
            Instruction::DecrementZ => pointers[2] -= 1,
            Instruction::IncrementW => pointers[3] += 1,
            Instruction::DecrementW => pointers[3] -= 1,
            Instruction::Increment  => tape[pointers[0]][pointers[1]][pointers[2]][pointers[3]] += 1,
            Instruction::Decrement  => tape[pointers[0]][pointers[1]][pointers[2]][pointers[3]] -= 1,
            Instruction::Write      => print!("{}", tape[pointers[0]][pointers[1]][pointers[2]][pointers[3]] as char),
            
            Instruction::Read       => {
                let mut input: [u8; 1] = [0; 1];

                std::io::stdin().read_exact(&mut input).expect("Failed to read data.");

                tape[pointers[0]][pointers[1]][pointers[2]][pointers[3]] = input[0];
            },
            
            Instruction::Loop(instructions) => {
                while tape[pointers[0]][pointers[1]][pointers[2]][pointers[3]] != 0 {
                    run(&instructions, tape, pointers);
                }
            },
        };
    }
}