use std::env::Args;

enum Registers {
        R0 = 0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        PC,     // program counter
        COND,   // condition flag (see Conditions)
        COUNT,  // number of registers
}

enum Opcodes {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

/* conditions flags provide information about the last calculation and
 * are set as values of Register::COND.
*/
enum Conditions {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}


/* `execute` fn executes commandline arguments
 */
pub fn execute(mut args: Args) {
    /* initialize the PC to starting position */
    let mut reg = [0 as u16; (Registers::COUNT as usize)];
    let mut memory = [0 as u16; (u16::max_value() as usize)];

    reg[(Registers::PC as usize)] = 0x3000;

    let mut running = 1;
    while running != 0 {
        running = running - 1;
    }

}
