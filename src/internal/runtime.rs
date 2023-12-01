use std::path::PathBuf;

use crate::parser::{Instruction, Parser};

use super::Program;

pub struct Runtime {
    program: Program,
    parser: Parser,
    instructions: Vec<Instruction>,
}

#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            parser: Parser::new(),
            instructions: Vec::new(),
        }
    }
    pub fn run_file(&mut self, path: PathBuf) {
        let input = std::fs::read_to_string(path).unwrap();
        self.run(input);
    }
    pub fn run(&mut self, input: String) {
        self.instructions = self.parser.parse(input);
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Increment => {
                    self.program.increment();
                }
                Instruction::Decrement => match self.program.decrement() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Illegal instruction: {}", e);
                    }
                },
                Instruction::MoveForward => {
                    self.program.move_forward();
                }
                Instruction::MoveBackward => {
                    self.program.move_backward();
                }
                Instruction::Output => {
                    self.program.print();
                }
                Instruction::Input => {
                    self.program.input();
                }
                Instruction::Illegal => {
                    eprintln!("Illegal instruction, program terminated.");
                    std::process::exit(1);
                }
            }
        }
    }
}
