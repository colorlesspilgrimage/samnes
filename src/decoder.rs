enum LoadStore {
    /// Load Accumulator (N,Z)
    LDA,
    /// Load X Register (N,Z)
    LDX,
    /// Load Y Register (N,Z)
    LDY,
    /// Store accumulator
    STA,
    /// Store X Register
    STX,
    /// Store Y Register
    STY
}

enum RegTran {
    /// Transfer accumulator to X register (N,Z)
    TAX,
    /// Transfer accumulator to Y register (N,Z)
    TAY,
    /// Transfer X register to accumulator (N,Z)
    TXA,
    /// Transfer Y register to accumulator (N,Z)
    TYA
}

enum StackOps {
    /// Transfer stack pointer to X register (N,Z) 
    TSX,
    /// Transfer X register to stack pointer 
    TXS,
    /// Push accumulator onto stack 
    PHA,
    /// Push processor status onto stack 
    PHP,
    /// Pull accumulator from stack  (N,Z)
    PLA,
    /// Pull processor status from stack (All)
    PLP
}

enum Logical {
    /// Logical AND (N,Z)
    AND,
    /// Exclusive OR (N,Z)
    EOR,
    /// Logical inclusive OR (N,Z)    
    ORA,
    /// Bit test (N,V,Z)
    BIT
}

enum Arith {
    /// Add with carry (N,V,Z,C) 
    ADC,
    /// Subtract with carry (N,V,C,Z)
    SBC,
    /// Compare accumulator (N,Z,C)
    CMP,
    /// Compare X Register (N,Z,C) 
    CPX,
    /// Compare Y Register (N,Z,C)
    CPY
}

enum IncDec {
    /// Increment a memory location (N,Z)
    INC,
    /// Increment the X register (N,Z)
    INX,
    /// Increment the Y register (N,Z)
    INY,
    /// Decrement a memory location (N,Z)
    DEC,
    /// Decrement the X register (N,Z)
    DEX,
    /// Decrement the Y register (N,Z)
    DEY
}

enum Shifts {
    /// Arithmetic shift left (N,Z,C) 
    ASL,
    /// Logical shift right (N,Z,C) 
    LSR,
    /// Rotate left (N,Z,C) 
    ROL,
    /// Rotate right (N,Z,C)
    ROR
}

enum JmpCall {
    /// Jump to another location 
    JMP,
    /// Jump to a subroutine
    JSR,
    /// Return from subroutine
    RTS
}

enum Branch {
    /// Branch if carry flag clear
    BCC,
    /// Branch if carry flag set
    BCS,
    /// Branch if zero flag set 
    BEQ,
    /// Branch if negative flag set 
    BMI,
    /// Branch if zero flag clear 
    BNE,
    /// Branch if negative flag is clear 
    BPL,
    /// Branch if overflow flag is clear 
    BVC,
    /// Branch if overflow flag is set
    BVS
}

enum StatusFlagChg {
    /// Clear carry flag (C) 
    CLC,
    /// Clear decimal mode flag (D)
    CLD,
    /// Clear interrupt disable flag (I)
    CLI,
    /// Clear overflow flag (V)
    CLV,
    /// Set carry flag (C) 
    SEC,
    /// Set decimal mode flag (D) 
    SED,
    /// Set interrupt disable flag (I)
    SEI
}

enum SysFunc {
    /// Force an interrupt (B) 
    BRK,
    /// No operation
    NOP,
    /// Return from interupt (All)
    RTI
}

enum AddressMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IdxIndirect,
    IndirectIdx
}

/// Enum to contain all of our categories of instructions.
enum Instruction {
    SysFunc(SysFunc, AddressMode),
    StatusFlagChg(StatusFlagChg, AddressMode),
    Branch(Branch, AddressMode),
    JmpCall(JmpCall, AddressMode),
    Shifts(Shifts, AddressMode),
    IncDec(IncDec, AddressMode),
    Arith(Arith, AddressMode),
    Logical(Logical, AddressMode),
    StackOps(StackOps, AddressMode),
    RegTran(RegTran, AddressMode),
    LoadStore(LoadStore, AddressMode),
}

static INSTRUCTION_TABLE: [(Instruction, u8); 34] = [
    (Instruction::SysFunc(SysFunc::BRK, AddressMode::Implied), 0x00),
    (Instruction::SysFunc(SysFunc::NOP, AddressMode::Implied), 0xEA),
    (Instruction::SysFunc(SysFunc::RTI, AddressMode::Implied), 0x40),
    (Instruction::StatusFlagChg(StatusFlagChg::CLC, AddressMode::Implied), 0x18),
    (Instruction::StatusFlagChg(StatusFlagChg::CLD, AddressMode::Implied), 0xD8),
    (Instruction::StatusFlagChg(StatusFlagChg::CLI, AddressMode::Implied), 0x58),
    (Instruction::StatusFlagChg(StatusFlagChg::CLV, AddressMode::Implied), 0xB8),
    (Instruction::StatusFlagChg(StatusFlagChg::SEC, AddressMode::Implied), 0x38),
    (Instruction::StatusFlagChg(StatusFlagChg::SED, AddressMode::Implied), 0xF8),
    (Instruction::StatusFlagChg(StatusFlagChg::SEI, AddressMode::Implied), 0x78),
    (Instruction::Branch(Branch::BCC, AddressMode::Relative), 0x90),
    (Instruction::Branch(Branch::BCS, AddressMode::Relative), 0xB0),
    (Instruction::Branch(Branch::BEQ, AddressMode::Relative), 0xF0),
    (Instruction::Branch(Branch::BMI, AddressMode::Relative), 0x30),
    (Instruction::Branch(Branch::BNE, AddressMode::Relative), 0xD0),
    (Instruction::Branch(Branch::BPL, AddressMode::Relative), 0x10),
    (Instruction::Branch(Branch::BVC, AddressMode::Relative), 0x50),
    (Instruction::Branch(Branch::BVS, AddressMode::Relative), 0x70),
    (Instruction::JmpCall(JmpCall::JMP, AddressMode::Absolute), 0x4C),
    (Instruction::JmpCall(JmpCall::JMP, AddressMode::Indirect), 0x6C),
    (Instruction::JmpCall(JmpCall::JSR, AddressMode::Absolute), 0x20),
    (Instruction::JmpCall(JmpCall::RTS, AddressMode::Implied), 0x60),
    (Instruction::Shifts(Shifts::ASL, AddressMode::Accumulator), 0x0A),
    (Instruction::Shifts(Shifts::LSR, AddressMode::Accumulator), 0x0A),
    (Instruction::Shifts(Shifts::ROL, AddressMode::Accumulator), 0x0A),
    (Instruction::Shifts(Shifts::ROR, AddressMode::Accumulator), 0x0A),
    (Instruction::Shifts(Shifts::ASL, AddressMode::ZeroPage), 0x06),
    (Instruction::Shifts(Shifts::LSR, AddressMode::ZeroPage), 0x46),
    (Instruction::Shifts(Shifts::ROL, AddressMode::ZeroPage), 0x26),
    (Instruction::Shifts(Shifts::ROR, AddressMode::ZeroPage), 0x66),
    (Instruction::Shifts(Shifts::ASL, AddressMode::ZeroPageX), 0x16),
    (Instruction::Shifts(Shifts::LSR, AddressMode::ZeroPageX), 0x56),
    (Instruction::Shifts(Shifts::ROL, AddressMode::ZeroPageX), 0x36),
    (Instruction::Shifts(Shifts::ROR, AddressMode::ZeroPageX), 0x76),
];

static SHIFTS_ABS: [u8; 4] = [0x0E, 0x4E, 0x2E, 0x6E];
static SHIFTS_ABS_X: [u8; 4] = [0x1E, 0x5E, 0x3E, 0x7E];
static INC_DEC_IMPLIED: [u8; 4] = [0xCA, 0x88, 0xE8, 0xC8];
static INC_DEC_ZERO_PAGE: [u8; 3] = [0xC6, 0x88, 0xE6];
static INC_DEC_ZERO_PAGE_X: [u8; 2] = [0xD6, 0xF6];
static INC_DEC_ABS: [u8; 2] = [0xCE, 0xEE];
static INC_DEC_ABS_X: [u8; 2] = [0xDE, 0xFE];
static ARITH_IMMEDIATE: [u8; 5] = [0x69,0xC9,0xE0,0xC0,0xE9];
static ARITH_ZERO_PAGE: [u8; 5] = [0x65, 0xC5, 0xE4, 0xC4, 0xE5];
static ARTIH_ZERO_PAGE_X: [u8; 3] = [0x75, 0xD5, 0xF5];
static ARTIH_ABS: [u8; 5] = [0x6D, 0xCD, 0xEC, 0xCC, 0xED];
static ARITH_ABS_X: [u8; 3] = [0x7D, 0xDD, 0xFD];
static ARITH_ABS_Y: [u8; 3] = [0x79, 0xD9, 0xF9];
static ARITH_INDIRECT_X: [u8; 3] = [0x61, 0xC1, 0xE1];
static ARITH_INDIRECT_Y: [u8; 3] = [0x71, 0xD1, 0xF1];
static LOGICAL_IMMEDIATE: [u8; 4] = [0x29, 0x24, 0x49, 0x09];
static LOGICAL_ZERO_PAGE: [u8; 3] = [0x25,0x45, 0x05];
static LOGICAL_ZERO_PAGE_X: [u8; 3] = [0x35, 0x55, 0x15];
static LOGICAL_ABS: [u8; 4] = [0x2D, 0x2C, 0x4D, 0x0D];
static LOGICAL_ABS_X: [u8; 3] = [0x3D, 0x5D, 0x1D];
static LOGICAL_ABS_Y: [u8; 3] = [0x39, 0x59, 0x19];
static LOGICAL_INDIRECT_X: [u8; 3] = [0x21, 0x41, 0x01];
static LOGICAL_INDIRECT_Y: [u8; 3] = [0x31, 0x51,0x11];
static STACK_OPS: [u8; 6] = [0x48,0x08,0x68,0x28,0xBA,0x9A];
static REG_TRAN: [u8; 4] = [0xAA,0xA8,0x98,0x8A];
static LOAD_STORE_IMMEDIATE: [u8; 3] = [0xA9, 0xA2, 0xA0];
static LOAD_STORE_ZERO_PAGE: [u8; 6] = [0xA5, 0xA6, 0xA4, 0x85, 0x86, 0x84];
static LOAD_STORE_ZERO_PAGE_X: [u8; 4] = [0xB5, 0xB4, 0x95, 0x94];
static LOAD_STORE_ZERO_PAGE_Y: [u8; 2] = [0xB6, 0x96];
static LOAD_STORE_ABSOLUTE: [u8; 6] = [0xAD, 0xAE, 0xAC, 0x8D, 0x8E, 0x8C];
static LOAD_STORE_ABSOLUTE_X: [u8; 3] = [0xBD, 0xBC, 0x9D];
static LOAD_STORE_ABSOLUTE_Y: [u8; 3] = [0xB9, 0xBE, 0x99];
static LOAD_STORE_INDIRECT_X: [u8; 2] = [0xA1, 0x81];
static LOAD_STORE_INDIRECT_Y: [u8; 2] = [0xB1, 0x91];


// this function won't always be public...
pub fn decode(d: &Vec<u8>) /*-> Vec<Instruction>*/ {

    for b in d {
        if REG_TRAN.contains(b) {
            println!("GOT EM - REG_TRAN SPOTTED!!");
        }
    }
    //return Vec::new();
}
