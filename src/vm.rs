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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_has_memory () {
        let vm = VM::new();
        assert_eq!(vm.memory[0], 0);
    }
}