use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct RegisterIndex(pub u32);

const REGISTER_COUNT: usize = 4;

const REGISTERS: [&str; REGISTER_COUNT] = [
    "%r8", "%r9", "%r10", "%r11"
];

#[derive(Clone, Copy, Debug)]
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

    pub fn free_all(&mut self) {
        for i in 0..REGISTER_COUNT {
            self.freereg[i] = true;
        }
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