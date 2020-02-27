use crate::instructions::OpCode;

pub fn lex(source: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '>' => Some(OpCode::IncrementX),
            '<' => Some(OpCode::DecrementX),
            '^' => Some(OpCode::IncrementY),
            'v' => Some(OpCode::DecrementY),
            '*' => Some(OpCode::IncrementZ),
            'o' => Some(OpCode::DecrementZ),
            '@' => Some(OpCode::IncrementW),
            '?' => Some(OpCode::DecrementW),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _   => None,
        };

        match op {
            Some(op) => operations.push(op),
            None     => (),
        }
    }

    operations
}