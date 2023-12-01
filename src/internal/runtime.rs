use std::io::{Read, Write};

pub struct Runtime {
    pub memory: [Option<u8>; 65536],
    pub pointer: usize,
}

#[allow(dead_code)]
impl Runtime {
    pub fn new() -> Self {
        Self {
            memory: [None; 65536],
            pointer: 0,
        }
    }
    pub fn move_backward(&mut self) {
        if self.pointer == 0 {
            // note: underflow is a feature!
            self.pointer = self.memory.len() - 1;
            return;
        }
        self.pointer -= 1;
    }
    pub fn move_forward(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            // note: overflow is a feature!
            self.pointer = 0;
            return;
        }
        self.pointer += 1;
    }
    pub fn get(&self) -> Result<u8, std::io::Error> {
        if let Some(value) = self.memory[self.pointer] {
            Ok(value)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Memory set to None, you cannot read from it.",
            ))
        }
    }
    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer] = Some(value);
    }
    pub fn print(&self) {
        if let Err(e) = std::io::stdout().write_all(&[self.get().unwrap()]) {
            eprintln!("Error: {}", e);
        }
    }
    pub fn input(&mut self) {
        let mut buff: u8 = 0;
        if let Err(e) = std::io::stdin().read_exact(std::slice::from_mut(&mut buff)) {
            eprintln!("Error: {}", e);
        }
        self.set(buff);
    }
}
