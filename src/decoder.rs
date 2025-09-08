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

fn decode() {

}
