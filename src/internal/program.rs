use std::io::{Read, Write};

pub struct Program {
    pub memory: [Option<u8>; 65536],
    pub pointer: usize,
}

#[allow(dead_code)]
impl Program {
    pub fn new() -> Self {
        Self {
            memory: [None; 65536],
            pointer: 0,
        }
    }
    /// Moves the pointer backward. (-1)
    pub fn move_backward(&mut self) {
        if self.pointer == 0 {
            // note: underflow is a feature!
            self.pointer = self.memory.len() - 1;
            return;
        }
        self.pointer -= 1;
    }
    /// Moves the pointer forward. (+1)
    pub fn move_forward(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            // note: overflow is a feature!
            self.pointer = 0;
            return;
        }
        self.pointer += 1;
    }

    /// Decrements the value AT the pointer.
    pub fn decrement(&mut self) -> Result<(), std::io::Error> {
        if let Some(value) = self.memory[self.pointer] {
            if value == 0 {
                self.memory[self.pointer] = Some(u8::MAX);
                return Ok(());
            }
            self.memory[self.pointer] = Some(value - 1);
            return Ok(());
        }
        // this is only to bother people
        Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Memory set to None, you cannot decrement from it. Try to increment the value to set to one and then decrement."
        ))
    }

    /// Increments the value AT the pointer.
    pub fn increment(&mut self) {
        if let Some(value) = self.memory[self.pointer] {
            if value == u8::MAX {
                self.memory[self.pointer] = Some(0);
                return;
            }
            self.memory[self.pointer] = Some(value + 1);
        } else {
            self.memory[self.pointer] = Some(1);
        }
    }

    /// Gets the value AT the pointer
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

    /// Sets the value AT the pointer
    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer] = Some(value);
    }

    /// Prints the value AT the pointer
    pub fn print(&self) {
        print!("{}", self.get().unwrap() as char);
        let _ = std::io::stdout().flush();
    }

    /// Reads a byte from stdin and sets the value AT the pointer
    pub fn input(&mut self) {
        let mut buff = [0; 1];
        match std::io::stdin().read(&mut buff) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read from stdin: {}", e);
            }
        }
        self.set(buff[0]);
        let _ = std::io::stdout().flush();
    }
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn decrement_none() {
        let mut runtime = super::Program::new();
        assert!(runtime.decrement().is_err());
    }
    #[test]
    fn decrement_zero() {
        let mut runtime = super::Program::new();
        runtime.set(0);
        assert!(runtime.decrement().is_ok());
    }
    #[test]
    fn get_none() {
        let runtime = super::Program::new();
        assert!(runtime.get().is_err());
    }
}
