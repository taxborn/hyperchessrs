use crate::instructions::OpCode;
use crate::instructions::Instruction;

pub fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
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
            };
        } else {
            match op {
                OpCode::LoopBegin => loop_stack += 1,

                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[loop_start+1 .. i].to_vec())));
                    }
                },

                _ => (),
            };
        }
    }

    if loop_stack != 0 {
        panic!("Loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}