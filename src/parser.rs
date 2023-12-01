pub enum Instruction {
    Increment,
    Decrement,
    MoveForward,
    MoveBackward,
    Output,
    Input,
    Illegal,
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse(&self, input: String) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        for character in input.chars() {
            match character {
                'm' => {
                    instructions.push(Instruction::Increment);
                }
                'a' => {
                    instructions.push(Instruction::Decrement);
                }
                'e' => {
                    instructions.push(Instruction::MoveForward);
                }
                'i' => {
                    instructions.push(Instruction::MoveBackward);
                }
                '👍' => {
                    instructions.push(Instruction::Output);
                }
                ' ' => {
                    instructions.push(Instruction::Input);
                }
                '\n' => {
                    continue;
                }
                _ => {
                    instructions.push(Instruction::Illegal);
                }
            }
        }
        instructions
    }
}
