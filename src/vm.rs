use crate::flags;

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

pub enum Instruction {
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

fn mem_read(addr: u16) -> u16 {
    // read from memory (you're gonna need that) and parse it to an instruction
    0 // magic number. should be an instruction (above)
}

pub struct VM {
    memory: [u16; u16::MAX as usize],
    registers: [u16; Registers::COUNT as usize],
    running: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; u16::MAX as usize],
            registers: [0; Registers::COUNT as usize],
            running: false
        }
    }

    pub fn run(&mut self) {
        // load arguments
        // setup

        let pc_start = 0x3000;
        self.registers[Registers::PC as usize] = pc_start;

        // set to starting Position

        while self.running {

            let opcode = mem_read(self.registers[Registers::PC as usize]);
            self.registers[Registers::PC as usize] += 1;
            // switch on instruction

            let instruction: Instruction= Instruction::Branch;
            match instruction {
                Instruction::Branch => {},
                Instruction::Add => {},
                Instruction::Load => {},
                Instruction::Store => {},
                Instruction::JumpSubroutineInstruction => {},
                Instruction::And => {},
                Instruction::LoadRegister => {},
                Instruction::StoreRegister => {},
                Instruction::RTI => {}, // unused?
                Instruction::Not => {},
                Instruction::LoadIndirect => {},
                Instruction::StoreIndirect => {},
                Instruction::Jump => {},
                Instruction::Reserved => {},
                Instruction::LoadEffectiveAddress => {},
                Instruction::Trap => {},
                _ => println!("I'm running!")
            }
        }

        // shutdown

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