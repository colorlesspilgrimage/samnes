use crate::decoder::{self, AddressMode, Mnemonic};
use std::fmt;

// ... (flags module is unchanged) ...

pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub ac: u8,
    pub idx: u8,
    pub idy: u8,
    pub status: u8,
    memory: [u8; 0x10000],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0, sp: 0, ac: 0, idx: 0, idy: 0, status: 0,
            memory: [0; 0x10000],
        }
    }

    // ... (All the old methods are restored here) ...
    // ... mem_read, mem_write, load, reset, step, etc. ...
    // ... These methods will use `self.memory` directly ...
}

impl fmt::Display for CPU {
    // ... (Unchanged) ...
}
