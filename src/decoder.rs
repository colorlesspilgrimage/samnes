#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    // Load/Store
    LDA, LDX, LDY, STA, STX, STY,
    // Register Transfer
    TAX, TAY, TXA, TYA,
    // Stack
    TSX, TXS, PHA, PHP, PLA, PLP,
    // Logical
    AND, EOR, ORA, BIT,
    // Arithmetic
    ADC, SBC, CMP, CPX, CPY,
    // Increment/Decrement
    INC, INX, INY, DEC, DEX, DEY,
    // Shifts
    ASL, LSR, ROL, ROR,
    // Jumps/Calls
    JMP, JSR, RTS,
    // Branches
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    // Status Flag Changes
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    // System
    BRK, NOP, RTI,

    // Unofficial Opcodes
    ALR, ANC, ARR, AXS, DCP, ISC, JAM, LAS, LAX, RLA, RRA, SAX, SLO, SRE, AHX, SHX, SHY, TAS, XAA,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddressMode {
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
    IndirectY,
}

#[derive(Copy, Clone, Debug)]
pub struct InstructionInfo {
    pub mnemonic: Mnemonic,
    pub mode: AddressMode,
    pub bytes: u8,
    pub cycles: u8,
}

macro_rules! opcodes {
    ($($opcode:expr => ($mnemonic:ident, $mode:ident, $bytes:expr, $cycles:expr)),* $(,)*) => {
        {
            let mut map = [None; 256];
            $(
                map[$opcode] = Some(InstructionInfo {
                    mnemonic: Mnemonic::$mnemonic,
                    mode: AddressMode::$mode,
                    bytes: $bytes,
                    cycles: $cycles,
                });
            )*
            map
        }
    };
}

// Using a macro to define the opcode map concisely.
// This is a direct lookup table for all 256 possible opcodes.
// See http://www.obelisk.me.uk/6502/reference.html for a complete reference.
// Cycle counts are base values and may take longer (e.g., page boundary crossings).
pub static OPCODE_MAP: [Option<InstructionInfo>; 256] = opcodes! {
    // Official Opcodes
    0x00 => (BRK, Implied, 1, 7),
    0x01 => (ORA, IndirectX, 2, 6),
    0x05 => (ORA, ZeroPage, 2, 3),
    0x06 => (ASL, ZeroPage, 2, 5),
    0x08 => (PHP, Implied, 1, 3),
    0x09 => (ORA, Immediate, 2, 2),
    0x0A => (ASL, Accumulator, 1, 2),
    0x0D => (ORA, Absolute, 3, 4),
    0x0E => (ASL, Absolute, 3, 6),
    0x10 => (BPL, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0x11 => (ORA, IndirectY, 2, 5), // +1 if page crossed
    0x15 => (ORA, ZeroPageX, 2, 4),
    0x16 => (ASL, ZeroPageX, 2, 6),
    0x18 => (CLC, Implied, 1, 2),
    0x19 => (ORA, AbsoluteY, 3, 4), // +1 if page crossed
    0x1D => (ORA, AbsoluteX, 3, 4), // +1 if page crossed
    0x1E => (ASL, AbsoluteX, 3, 7),
    0x20 => (JSR, Absolute, 3, 6),
    0x21 => (AND, IndirectX, 2, 6),
    0x24 => (BIT, ZeroPage, 2, 3),
    0x25 => (AND, ZeroPage, 2, 3),
    0x26 => (ROL, ZeroPage, 2, 5),
    0x28 => (PLP, Implied, 1, 4),
    0x29 => (AND, Immediate, 2, 2),
    0x2A => (ROL, Accumulator, 1, 2),
    0x2C => (BIT, Absolute, 3, 4),
    0x2D => (AND, Absolute, 3, 4),
    0x2E => (ROL, Absolute, 3, 6),
    0x30 => (BMI, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0x31 => (AND, IndirectY, 2, 5), // +1 if page crossed
    0x35 => (AND, ZeroPageX, 2, 4),
    0x36 => (ROL, ZeroPageX, 2, 6),
    0x38 => (SEC, Implied, 1, 2),
    0x39 => (AND, AbsoluteY, 3, 4), // +1 if page crossed
    0x3D => (AND, AbsoluteX, 3, 4), // +1 if page crossed
    0x3E => (ROL, AbsoluteX, 3, 7),
    0x40 => (RTI, Implied, 1, 6),
    0x41 => (EOR, IndirectX, 2, 6),
    0x45 => (EOR, ZeroPage, 2, 3),
    0x46 => (LSR, ZeroPage, 2, 5),
    0x48 => (PHA, Implied, 1, 3),
    0x49 => (EOR, Immediate, 2, 2),
    0x4A => (LSR, Accumulator, 1, 2),
    0x4C => (JMP, Absolute, 3, 3),
    0x4D => (EOR, Absolute, 3, 4),
    0x4E => (LSR, Absolute, 3, 6),
    0x50 => (BVC, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0x51 => (EOR, IndirectY, 2, 5), // +1 if page crossed
    0x55 => (EOR, ZeroPageX, 2, 4),
    0x56 => (LSR, ZeroPageX, 2, 6),
    0x58 => (CLI, Implied, 1, 2),
    0x59 => (EOR, AbsoluteY, 3, 4), // +1 if page crossed
    0x5D => (EOR, AbsoluteX, 3, 4), // +1 if page crossed
    0x5E => (LSR, AbsoluteX, 3, 7),
    0x60 => (RTS, Implied, 1, 6),
    0x61 => (ADC, IndirectX, 2, 6),
    0x65 => (ADC, ZeroPage, 2, 3),
    0x66 => (ROR, ZeroPage, 2, 5),
    0x68 => (PLA, Implied, 1, 4),
    0x69 => (ADC, Immediate, 2, 2),
    0x6A => (ROR, Accumulator, 1, 2),
    0x6C => (JMP, Indirect, 3, 5),
    0x6D => (ADC, Absolute, 3, 4),
    0x6E => (ROR, Absolute, 3, 6),
    0x70 => (BVS, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0x71 => (ADC, IndirectY, 2, 5), // +1 if page crossed
    0x75 => (ADC, ZeroPageX, 2, 4),
    0x76 => (ROR, ZeroPageX, 2, 6),
    0x78 => (SEI, Implied, 1, 2),
    0x79 => (ADC, AbsoluteY, 3, 4), // +1 if page crossed
    0x7D => (ADC, AbsoluteX, 3, 4), // +1 if page crossed
    0x7E => (ROR, AbsoluteX, 3, 7),
    0x81 => (STA, IndirectX, 2, 6),
    0x84 => (STY, ZeroPage, 2, 3),
    0x85 => (STA, ZeroPage, 2, 3),
    0x86 => (STX, ZeroPage, 2, 3),
    0x88 => (DEY, Implied, 1, 2),
    0x8A => (TXA, Implied, 1, 2),
    0x8C => (STY, Absolute, 3, 4),
    0x8D => (STA, Absolute, 3, 4),
    0x8E => (STX, Absolute, 3, 4),
    0x90 => (BCC, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0x91 => (STA, IndirectY, 2, 6),
    0x94 => (STY, ZeroPageX, 2, 4),
    0x95 => (STA, ZeroPageX, 2, 4),
    0x96 => (STX, ZeroPageY, 2, 4),
    0x98 => (TYA, Implied, 1, 2),
    0x99 => (STA, AbsoluteY, 3, 5),
    0x9A => (TXS, Implied, 1, 2),
    0x9D => (STA, AbsoluteX, 3, 5),
    0xA0 => (LDY, Immediate, 2, 2),
    0xA1 => (LDA, IndirectX, 2, 6),
    0xA2 => (LDX, Immediate, 2, 2),
    0xA4 => (LDY, ZeroPage, 2, 3),
    0xA5 => (LDA, ZeroPage, 2, 3),
    0xA6 => (LDX, ZeroPage, 2, 3),
    0xA8 => (TAY, Implied, 1, 2),
    0xA9 => (LDA, Immediate, 2, 2),
    0xAA => (TAX, Implied, 1, 2),
    0xAC => (LDY, Absolute, 3, 4),
    0xAD => (LDA, Absolute, 3, 4),
    0xAE => (LDX, Absolute, 3, 4),
    0xB0 => (BCS, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0xB1 => (LDA, IndirectY, 2, 5), // +1 if page crossed
    0xB4 => (LDY, ZeroPageX, 2, 4),
    0xB5 => (LDA, ZeroPageX, 2, 4),
    0xB6 => (LDX, ZeroPageY, 2, 4),
    0xB8 => (CLV, Implied, 1, 2),
    0xB9 => (LDA, AbsoluteY, 3, 4), // +1 if page crossed
    0xBA => (TSX, Implied, 1, 2),
    0xBC => (LDY, AbsoluteX, 3, 4), // +1 if page crossed
    0xBD => (LDA, AbsoluteX, 3, 4), // +1 if page crossed
    0xBE => (LDX, AbsoluteY, 3, 4), // +1 if page crossed
    0xC0 => (CPY, Immediate, 2, 2),
    0xC1 => (CMP, IndirectX, 2, 6),
    0xC4 => (CPY, ZeroPage, 2, 3),
    0xC5 => (CMP, ZeroPage, 2, 3),
    0xC6 => (DEC, ZeroPage, 2, 5),
    0xC8 => (INY, Implied, 1, 2),
    0xC9 => (CMP, Immediate, 2, 2),
    0xCA => (DEX, Implied, 1, 2),
    0xCC => (CPY, Absolute, 3, 4),
    0xCD => (CMP, Absolute, 3, 4),
    0xCE => (DEC, Absolute, 3, 6),
    0xD0 => (BNE, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0xD1 => (CMP, IndirectY, 2, 5), // +1 if page crossed
    0xD5 => (CMP, ZeroPageX, 2, 4),
    0xD6 => (DEC, ZeroPageX, 2, 6),
    0xD8 => (CLD, Implied, 1, 2),
    0xD9 => (CMP, AbsoluteY, 3, 4), // +1 if page crossed
    0xDD => (CMP, AbsoluteX, 3, 4), // +1 if page crossed
    0xDE => (DEC, AbsoluteX, 3, 7),
    0xE0 => (CPX, Immediate, 2, 2),
    0xE1 => (SBC, IndirectX, 2, 6),
    0xE4 => (CPX, ZeroPage, 2, 3),
    0xE5 => (SBC, ZeroPage, 2, 3),
    0xE6 => (INC, ZeroPage, 2, 5),
    0xE8 => (INX, Implied, 1, 2),
    0xE9 => (SBC, Immediate, 2, 2),
    0xEA => (NOP, Implied, 1, 2),
    0xEC => (CPX, Absolute, 3, 4),
    0xED => (SBC, Absolute, 3, 4),
    0xEE => (INC, Absolute, 3, 6),
    0xF0 => (BEQ, Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page
    0xF1 => (SBC, IndirectY, 2, 5), // +1 if page crossed
    0xF5 => (SBC, ZeroPageX, 2, 4),
    0xF6 => (INC, ZeroPageX, 2, 6),
    0xF8 => (SED, Implied, 1, 2),
    0xF9 => (SBC, AbsoluteY, 3, 4), // +1 if page crossed
    0xFD => (SBC, AbsoluteX, 3, 4), // +1 if page crossed
    0xFE => (INC, AbsoluteX, 3, 7),

    // Unofficial Opcodes
    0x02 => (JAM, Implied, 1, 0), // Halts CPU
    0x03 => (SLO, IndirectX, 2, 8),
    0x04 => (NOP, ZeroPage, 2, 3),
    0x07 => (SLO, ZeroPage, 2, 5),
    0x0B => (ANC, Immediate, 2, 2),
    0x0C => (NOP, Absolute, 3, 4),
    0x0F => (SLO, Absolute, 3, 6),
    0x12 => (JAM, Implied, 1, 0),
    0x13 => (SLO, IndirectY, 2, 8),
    0x14 => (NOP, ZeroPageX, 2, 4),
    0x17 => (SLO, ZeroPageX, 2, 6),
    0x1A => (NOP, Implied, 1, 2),
    0x1B => (SLO, AbsoluteY, 3, 7),
    0x1C => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0x1F => (SLO, AbsoluteX, 3, 7),
    0x22 => (JAM, Implied, 1, 0),
    0x23 => (RLA, IndirectX, 2, 8),
    0x27 => (RLA, ZeroPage, 2, 5),
    0x2B => (ANC, Immediate, 2, 2),
    0x2F => (RLA, Absolute, 3, 6),
    0x32 => (JAM, Implied, 1, 0),
    0x33 => (RLA, IndirectY, 2, 8),
    0x34 => (NOP, ZeroPageX, 2, 4),
    0x37 => (RLA, ZeroPageX, 2, 6),
    0x3A => (NOP, Implied, 1, 2),
    0x3B => (RLA, AbsoluteY, 3, 7),
    0x3C => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0x3F => (RLA, AbsoluteX, 3, 7),
    0x42 => (JAM, Implied, 1, 0),
    0x43 => (SRE, IndirectX, 2, 8),
    0x44 => (NOP, ZeroPage, 2, 3),
    0x47 => (SRE, ZeroPage, 2, 5),
    0x4B => (ALR, Immediate, 2, 2),
    0x4F => (SRE, Absolute, 3, 6),
    0x52 => (JAM, Implied, 1, 0),
    0x53 => (SRE, IndirectY, 2, 8),
    0x54 => (NOP, ZeroPageX, 2, 4),
    0x57 => (SRE, ZeroPageX, 2, 6),
    0x5A => (NOP, Implied, 1, 2),
    0x5B => (SRE, AbsoluteY, 3, 7),
    0x5C => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0x5F => (SRE, AbsoluteX, 3, 7),
    0x62 => (JAM, Implied, 1, 0),
    0x63 => (RRA, IndirectX, 2, 8),
    0x64 => (NOP, ZeroPage, 2, 3),
    0x67 => (RRA, ZeroPage, 2, 5),
    0x6B => (ARR, Immediate, 2, 2),
    0x6F => (RRA, Absolute, 3, 6),
    0x72 => (JAM, Implied, 1, 0),
    0x73 => (RRA, IndirectY, 2, 8),
    0x74 => (NOP, ZeroPageX, 2, 4),
    0x77 => (RRA, ZeroPageX, 2, 6),
    0x7A => (NOP, Implied, 1, 2),
    0x7B => (RRA, AbsoluteY, 3, 7),
    0x7C => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0x7F => (RRA, AbsoluteX, 3, 7),
    0x80 => (NOP, Immediate, 2, 2),
    0x82 => (NOP, Immediate, 2, 2),
    0x83 => (SAX, IndirectX, 2, 6),
    0x87 => (SAX, ZeroPage, 2, 3),
    0x89 => (NOP, Immediate, 2, 2),
    0x8B => (XAA, Immediate, 2, 2), // Highly unstable
    0x8F => (SAX, Absolute, 3, 4),
    0x92 => (JAM, Implied, 1, 0),
    0x93 => (AHX, IndirectY, 2, 6),
    0x97 => (SAX, ZeroPageY, 2, 4),
    0x9B => (TAS, AbsoluteY, 3, 5),
    0x9C => (SHY, AbsoluteX, 3, 5),
    0x9E => (SHX, AbsoluteY, 3, 5),
    0x9F => (AHX, AbsoluteY, 3, 5),
    0xA3 => (LAX, IndirectX, 2, 6),
    0xA7 => (LAX, ZeroPage, 2, 3),
    0xAB => (LAX, Immediate, 2, 2),
    0xAF => (LAX, Absolute, 3, 4),
    0xB2 => (JAM, Implied, 1, 0),
    0xB3 => (LAX, IndirectY, 2, 5), // +1 if page crossed
    0xB7 => (LAX, ZeroPageY, 2, 4),
    0xBB => (LAS, AbsoluteY, 3, 4), // +1 if page crossed
    0xBF => (LAX, AbsoluteY, 3, 4), // +1 if page crossed
    0xC2 => (NOP, Immediate, 2, 2),
    0xC3 => (DCP, IndirectX, 2, 8),
    0xC7 => (DCP, ZeroPage, 2, 5),
    0xCB => (AXS, Immediate, 2, 2),
    0xCF => (DCP, Absolute, 3, 6),
    0xD2 => (JAM, Implied, 1, 0),
    0xD3 => (DCP, IndirectY, 2, 8),
    0xD4 => (NOP, ZeroPageX, 2, 4),
    0xD7 => (DCP, ZeroPageX, 2, 6),
    0xDA => (NOP, Implied, 1, 2),
    0xDB => (DCP, AbsoluteY, 3, 7),
    0xDC => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0xDF => (DCP, AbsoluteX, 3, 7),
    0xE2 => (NOP, Immediate, 2, 2),
    0xE3 => (ISC, IndirectX, 2, 8),
    0xE7 => (ISC, ZeroPage, 2, 5),
    0xEB => (SBC, Immediate, 2, 2),
    0xEF => (ISC, Absolute, 3, 6),
    0xF2 => (JAM, Implied, 1, 0),
    0xF3 => (ISC, IndirectY, 2, 8),
    0xF4 => (NOP, ZeroPageX, 2, 4),
    0xF7 => (ISC, ZeroPageX, 2, 6),
    0xFA => (NOP, Implied, 1, 2),
    0xFB => (ISC, AbsoluteY, 3, 7),
    0xFC => (NOP, AbsoluteX, 3, 4), // +1 if page crossed
    0xFF => (ISC, AbsoluteX, 3, 7),
};

pub struct DecodedInstruction<'a> {
    pub info: &'a InstructionInfo,
    pub operand: Option<u16>,
}

/// Disassembles a bytecode stream into a vector of instructions.
/// This new function correctly handles operands and uses the efficient OPCODE_MAP for lookups.
pub fn disassemble(bytecode: &[u8]) -> Vec<DecodedInstruction> {
    let mut instructions = Vec::new();
    let mut pc = 0;
    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        if let Some(info) = &OPCODE_MAP[opcode as usize] {
            let operand = match info.bytes {
                2 => {
                    if pc + 1 < bytecode.len() {
                        Some(bytecode[pc + 1] as u16)
                    } else { None }
                },
                3 => {
                    if pc + 2 < bytecode.len() {
                        Some(u16::from_le_bytes([bytecode[pc + 1], bytecode[pc + 2]]))
                    } else { None }
                },
                _ => None,
            };

            instructions.push(DecodedInstruction { info, operand });
            pc += info.bytes as usize;
        } else {
            // Handle illegal/unknown opcode, just advance past it
            pc += 1;
        }
    }
    instructions
}