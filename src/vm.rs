pub struct VM {
    memory: [u16; usize.MAX],
    registers: [u16; 10],
}

impl VM {
    pub fn new() -> VM {
        VM {}
    }
}
