
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

pub enum Instruciton {
    Branch,
    Add,
    Load,
    Store,
    JumpSubroutineInstruction,
    And,
    LoadRegister,
    StoreRegister,
    RTI, // unused?
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved,
    LoadEffectiveAddress,
    Trap,
}

pub struct Flags {
    negative: bool,
    zero: bool,
    position: bool,
}

impl Flags {
    pub fn update(&mut self, reg: u16) {
        match reg {
            0x0000 => {
                self.negative = false;
                self.zero = true;
                self.position = false;
            }
            0x0001..=0x7fff => {
                self.negative = false;
                self.zero = false;
                self.position = true;
            }
            0x8000..=0xffff => {
                self.negative = true;
                self.zero = false;
                self.position = false;
            }
        }
    }
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
    }
}