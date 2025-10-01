#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
enum JmpCall {
    /// Jump to another location 
    JMP,
    /// Jump to a subroutine
    JSR,
    /// Return from subroutine
    RTS
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
enum SysFunc {
    /// Force an interrupt (B) 
    BRK,
    /// No operation
    NOP,
    /// Return from interupt (All)
    RTI
}

#[derive(Copy, Clone, Debug)]
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
    IndirectX,
    IndirectY
}

/// Enum to contain all of our categories of instructions.
#[derive(Copy, Clone, Debug)]
pub enum Instruction {
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

// we also need to somehow encode how many arguments each of these take to ensure that when we
// disassemble, we associate the correct arguments with each instruction
static INSTRUCTION_TABLE: [(Instruction, u8, u8); 150] = [
    (Instruction::SysFunc(SysFunc::BRK, AddressMode::Implied), 0x00, 0),
    (Instruction::SysFunc(SysFunc::NOP, AddressMode::Implied), 0xEA, 0),
    (Instruction::SysFunc(SysFunc::RTI, AddressMode::Implied), 0x40, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::CLC, AddressMode::Implied), 0x18, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::CLD, AddressMode::Implied), 0xD8, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::CLI, AddressMode::Implied), 0x58, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::CLV, AddressMode::Implied), 0xB8, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::SEC, AddressMode::Implied), 0x38, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::SED, AddressMode::Implied), 0xF8, 0),
    (Instruction::StatusFlagChg(StatusFlagChg::SEI, AddressMode::Implied), 0x78, 0),
    (Instruction::Branch(Branch::BCC, AddressMode::Relative), 0x90, 0),
    (Instruction::Branch(Branch::BCS, AddressMode::Relative), 0xB0, 0),
    (Instruction::Branch(Branch::BEQ, AddressMode::Relative), 0xF0, 0),
    (Instruction::Branch(Branch::BMI, AddressMode::Relative), 0x30, 0),
    (Instruction::Branch(Branch::BNE, AddressMode::Relative), 0xD0, 0),
    (Instruction::Branch(Branch::BPL, AddressMode::Relative), 0x10, 0),
    (Instruction::Branch(Branch::BVC, AddressMode::Relative), 0x50, 0),
    (Instruction::Branch(Branch::BVS, AddressMode::Relative), 0x70, 0),
    (Instruction::JmpCall(JmpCall::JMP, AddressMode::Absolute), 0x4C, 0),
    (Instruction::JmpCall(JmpCall::JMP, AddressMode::Indirect), 0x6C, 0),
    (Instruction::JmpCall(JmpCall::JSR, AddressMode::Absolute), 0x20, 0),
    (Instruction::JmpCall(JmpCall::RTS, AddressMode::Implied), 0x60, 0),
    (Instruction::Shifts(Shifts::ASL, AddressMode::Accumulator), 0x0A, 0),
    (Instruction::Shifts(Shifts::LSR, AddressMode::Accumulator), 0x0A, 0),
    (Instruction::Shifts(Shifts::ROL, AddressMode::Accumulator), 0x0A, 0),
    (Instruction::Shifts(Shifts::ROR, AddressMode::Accumulator), 0x0A, 0),
    (Instruction::Shifts(Shifts::ASL, AddressMode::ZeroPage), 0x06, 0),
    (Instruction::Shifts(Shifts::LSR, AddressMode::ZeroPage), 0x46, 0),
    (Instruction::Shifts(Shifts::ROL, AddressMode::ZeroPage), 0x26, 0),
    (Instruction::Shifts(Shifts::ROR, AddressMode::ZeroPage), 0x66, 0),
    (Instruction::Shifts(Shifts::ASL, AddressMode::ZeroPageX), 0x16, 0),
    (Instruction::Shifts(Shifts::LSR, AddressMode::ZeroPageX), 0x56, 0),
    (Instruction::Shifts(Shifts::ROL, AddressMode::ZeroPageX), 0x36, 0),
    (Instruction::Shifts(Shifts::ROR, AddressMode::ZeroPageX), 0x76, 0),
    (Instruction::Shifts(Shifts::ASL, AddressMode::Absolute), 0x0E, 0),
    (Instruction::Shifts(Shifts::LSR, AddressMode::Absolute), 0x4E, 0),
    (Instruction::Shifts(Shifts::ROL, AddressMode::Absolute), 0x2E, 0),
    (Instruction::Shifts(Shifts::ROR, AddressMode::Absolute), 0x6E, 0),
    (Instruction::Shifts(Shifts::ASL, AddressMode::AbsoluteX), 0x1E, 0),
    (Instruction::Shifts(Shifts::LSR, AddressMode::AbsoluteX), 0x5E, 0),
    (Instruction::Shifts(Shifts::ROL, AddressMode::AbsoluteX), 0x3E, 0),
    (Instruction::Shifts(Shifts::ROR, AddressMode::AbsoluteX), 0x7E, 0),
    (Instruction::IncDec(IncDec::DEX, AddressMode::Implied), 0xCA, 0),
    (Instruction::IncDec(IncDec::DEY, AddressMode::Implied), 0x88, 0),
    (Instruction::IncDec(IncDec::INX, AddressMode::Implied), 0xE8, 0),
    (Instruction::IncDec(IncDec::INY, AddressMode::Implied), 0xC8, 0),
    (Instruction::IncDec(IncDec::DEC, AddressMode::ZeroPage), 0xC6, 0),
    (Instruction::IncDec(IncDec::INC, AddressMode::ZeroPage), 0xE6, 0),
    (Instruction::IncDec(IncDec::DEC, AddressMode::ZeroPageX), 0xD6, 0),
    (Instruction::IncDec(IncDec::INC, AddressMode::ZeroPageX), 0xF6, 0),
    (Instruction::IncDec(IncDec::DEC, AddressMode::Absolute), 0xCE, 0),
    (Instruction::IncDec(IncDec::INC, AddressMode::Absolute), 0xEE, 0),
    (Instruction::IncDec(IncDec::DEC, AddressMode::AbsoluteX), 0xDE, 0),
    (Instruction::IncDec(IncDec::INC, AddressMode::AbsoluteX), 0xFE, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::Immediate), 0x69, 0),
    (Instruction::Arith(Arith::CMP, AddressMode::Immediate), 0xC9, 0),
    (Instruction::Arith(Arith::CPX, AddressMode::Immediate), 0xE0, 0),
    (Instruction::Arith(Arith::CPY, AddressMode::Immediate), 0xC0, 0),
    (Instruction::Arith(Arith::SBC, AddressMode::Immediate), 0xE9, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::ZeroPage), 0x65, 0),
    (Instruction::Arith(Arith::CMP, AddressMode::ZeroPage), 0xC5, 0),
    (Instruction::Arith(Arith::CPX, AddressMode::ZeroPage), 0xE4, 0),
    (Instruction::Arith(Arith::CPY, AddressMode::ZeroPage), 0xC4, 0),
    (Instruction::Arith(Arith::SBC, AddressMode::ZeroPage), 0xE9, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::ZeroPageX), 0x75, 0),
    (Instruction::Arith(Arith::CMP, AddressMode::ZeroPageX), 0xD5, 0),
    (Instruction::Arith(Arith::SBC, AddressMode::ZeroPageX), 0xF5, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::Absolute), 0x6D, 0),
    (Instruction::Arith(Arith::CMP, AddressMode::Absolute), 0xCD, 0),
    (Instruction::Arith(Arith::CPX, AddressMode::Absolute), 0xEC, 0),
    (Instruction::Arith(Arith::CPY, AddressMode::Absolute), 0xCC, 0),
    (Instruction::Arith(Arith::SBC, AddressMode::Absolute), 0xED, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteX), 0x7D, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteX), 0xDD, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteX), 0xFD, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteY), 0x79, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteY), 0xD9, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::AbsoluteY), 0xF9, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectX), 0x61, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectX), 0xC1, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectX), 0xE1, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectY), 0x71, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectY), 0xD1, 0),
    (Instruction::Arith(Arith::ADC, AddressMode::IndirectY), 0xF1, 0),
    (Instruction::Logical(Logical::AND, AddressMode::Immediate), 0x29, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::Immediate), 0x49, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::Immediate), 0x09, 0),
    (Instruction::Logical(Logical::AND, AddressMode::ZeroPage), 0x25, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::ZeroPage), 0x45, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::ZeroPage), 0x05, 0),
    (Instruction::Logical(Logical::AND, AddressMode::ZeroPageX), 0x35, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::ZeroPageX), 0x55, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::ZeroPageX), 0x15, 0),
    (Instruction::Logical(Logical::AND, AddressMode::Absolute), 0x2D, 0),
    (Instruction::Logical(Logical::BIT, AddressMode::Absolute), 0x2C, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::Absolute), 0x4D, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::Absolute), 0x0D, 0),
    (Instruction::Logical(Logical::AND, AddressMode::AbsoluteX), 0x3D, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::AbsoluteX), 0x5D, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::AbsoluteX), 0x1D, 0),
    (Instruction::Logical(Logical::AND, AddressMode::AbsoluteY), 0x39, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::AbsoluteY), 0x59, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::AbsoluteY), 0x19, 0),
    (Instruction::Logical(Logical::AND, AddressMode::IndirectX), 0x21, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::IndirectX), 0x41, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::IndirectX), 0x01, 0),
    (Instruction::Logical(Logical::AND, AddressMode::IndirectY), 0x31, 0),
    (Instruction::Logical(Logical::EOR, AddressMode::IndirectY), 0x51, 0),
    (Instruction::Logical(Logical::ORA, AddressMode::IndirectY), 0x11, 0),
    (Instruction::StackOps(StackOps::PHA, AddressMode::Implied), 0x48, 0),
    (Instruction::StackOps(StackOps::PHP, AddressMode::Implied), 0x08, 0),
    (Instruction::StackOps(StackOps::PLA, AddressMode::Implied), 0x68, 0),
    (Instruction::StackOps(StackOps::PLP, AddressMode::Implied), 0x28, 0),
    (Instruction::StackOps(StackOps::TSX, AddressMode::Implied), 0xBA, 0),
    (Instruction::StackOps(StackOps::TXS, AddressMode::Implied), 0x9A, 0),
    (Instruction::RegTran(RegTran::TAX, AddressMode::Implied), 0xAA, 0),
    (Instruction::RegTran(RegTran::TAY, AddressMode::Implied), 0xA8, 0),
    (Instruction::RegTran(RegTran::TYA, AddressMode::Implied), 0x98, 0),
    (Instruction::RegTran(RegTran::TXA, AddressMode::Implied), 0x8A, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::Immediate), 0xA9, 0),
    (Instruction::LoadStore(LoadStore::LDX, AddressMode::Immediate), 0xA2, 0),
    (Instruction::LoadStore(LoadStore::LDY, AddressMode::Immediate), 0xA0, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::ZeroPage), 0xA5, 0),
    (Instruction::LoadStore(LoadStore::LDX, AddressMode::ZeroPage), 0xA6, 0),
    (Instruction::LoadStore(LoadStore::LDY, AddressMode::ZeroPage), 0xA4, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::ZeroPage), 0x85, 0),
    (Instruction::LoadStore(LoadStore::STX, AddressMode::ZeroPage), 0x86, 0),
    (Instruction::LoadStore(LoadStore::STY, AddressMode::ZeroPage), 0x84, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::ZeroPageX), 0xB5, 0),
    (Instruction::LoadStore(LoadStore::LDY, AddressMode::ZeroPageX), 0xB4, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::ZeroPageX), 0x95, 0),
    (Instruction::LoadStore(LoadStore::STY, AddressMode::ZeroPageX), 0x94, 0),
    (Instruction::LoadStore(LoadStore::LDX, AddressMode::ZeroPageY), 0xB6, 0),
    (Instruction::LoadStore(LoadStore::STX, AddressMode::ZeroPageY), 0x96, 0),
    (Instruction::LoadStore(LoadStore::STY, AddressMode::Absolute), 0xAD, 0),
    (Instruction::LoadStore(LoadStore::LDX, AddressMode::Absolute), 0xAE, 0),
    (Instruction::LoadStore(LoadStore::LDY, AddressMode::Absolute), 0xAC, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::Absolute), 0x8D, 0),
    (Instruction::LoadStore(LoadStore::STX, AddressMode::Absolute), 0x8E, 0),
    (Instruction::LoadStore(LoadStore::STY, AddressMode::Absolute), 0x8C, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::AbsoluteX), 0xBD, 0),
    (Instruction::LoadStore(LoadStore::LDY, AddressMode::AbsoluteX), 0xBC, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::AbsoluteX), 0x9D, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::AbsoluteY), 0xB9, 0),
    (Instruction::LoadStore(LoadStore::LDX, AddressMode::AbsoluteY), 0xBE, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::AbsoluteY), 0x99, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::IndirectX), 0xA1, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::IndirectX), 0x81, 0),
    (Instruction::LoadStore(LoadStore::LDA, AddressMode::IndirectY), 0xB1, 0),
    (Instruction::LoadStore(LoadStore::STA, AddressMode::IndirectY), 0x91, 0),
];

// right now, this is pre-decompiling. for better performance in the future we might want to call
// this in real time to JIT decompile instead of putting the loading burden on the start.
pub fn decode(bytecode: &Vec<u8>) -> Vec<Instruction> {
    let mut v = Vec::new();
    for b in bytecode {
        for (ins, op, _) in INSTRUCTION_TABLE {
            if *b == op {
                v.push(ins);
            }
        }
    }
    return v;
}
