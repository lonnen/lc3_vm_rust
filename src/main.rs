enum Registers {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        PC,
        COND,
        COUNT,
}

enum Opcodes {
    BR,     // branch
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

fn main() {
    let mut reg: [u16; (Registers::COUNT as usize)];
    let mut memory: [u16; (u16::max_value() as usize)];

    println!("Hello, world!");
}
