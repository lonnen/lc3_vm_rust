pub struct VM {
    memory: [u16; u16::MAX as usize],
    registers: [u16; 10],
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; u16::MAX as usize],
            registers: [0; 10],
        }
    }
}
