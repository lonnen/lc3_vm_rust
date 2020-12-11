
enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, /* program counter */
    COND,
    COUNT,
}

pub struct VM {
    memory: [u16; u16::MAX as usize],
    registers: [u16; Registers::COUNT as usize],
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; u16::MAX as usize],
            registers: [0; Registers::COUNT as usize],
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