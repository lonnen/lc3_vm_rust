
enum Registers {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8, /* program counter */
    COND = 9,
    COUNT = 10,
}

pub struct VM {
    memory: [u16; u16::MAX as usize],
    registers: [u16; 11],
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; u16::MAX as usize],
            registers: [0; 11],
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

    #[test]
    fn it_has_registers () {
        let vm = VM::new();
        assert_eq!(vm.registers[Registers::R0 as usize], 0);
        assert_eq!(vm.registers[Registers::R1 as usize], 0);
        assert_eq!(vm.registers[Registers::R2 as usize], 0);
        assert_eq!(vm.registers[Registers::R3 as usize], 0);
        assert_eq!(vm.registers[Registers::R4 as usize], 0);
        assert_eq!(vm.registers[Registers::R5 as usize], 0);
        assert_eq!(vm.registers[Registers::R6 as usize], 0);
        assert_eq!(vm.registers[Registers::R7 as usize], 0);
        assert_eq!(vm.registers[Registers::PC as usize], 0);
        assert_eq!(vm.registers[Registers::COND as usize], 0);
        assert_eq!(vm.registers[Registers::COUNT as usize], 0);
    }
}