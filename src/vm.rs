pub struct VM {
    memory: [u16; usize::MAX],
    registers: [u16; 10],
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; usize::MAX],
            registers: [0; 10],
        }
    }
}