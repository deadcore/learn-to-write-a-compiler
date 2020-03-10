use std::fmt;
use std::io::{BufWriter, Write};

#[derive(Clone, Copy)]
pub struct RegisterIndex(pub u32);

const REGISTER_COUNT: usize = 6;

const REGISTERS: [&str; REGISTER_COUNT] = [
    "r7d", "r8d", "r9d", "r10d", "r11d", "r12d"
];

#[derive(Clone, Copy)]
pub struct Registers {
    freereg: [bool; REGISTER_COUNT]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            freereg: [true; REGISTER_COUNT]
        }
    }

    pub fn allocate_register(&mut self) -> RegisterIndex {
        for i in 0..self.freereg.len() {
            if self.freereg[i] {
                self.freereg[i] = false;
                return RegisterIndex(i as u32);
            }
        }
        panic!("Out of registers")
    }

    pub fn free_register(&mut self, register: RegisterIndex) {
        if self.freereg[register.0 as usize] {
            panic!("Error trying to free register: {}", register)
        }

        self.freereg[register.0 as usize] = true;
    }
}

impl RegisterIndex {
    pub fn name(&self) -> &str {
        REGISTERS[self.0 as usize]
    }
}

impl fmt::Display for RegisterIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}