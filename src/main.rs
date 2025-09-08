use std::env;
use std::fmt;
use std::fs;

struct CPU {
    pc: u16,
    sp: u8,
    stack: [u8; 256],
    ac: u8,
    idx: u8,
    idy: u8,
    cf: bool,
    zf: bool,
    id: bool,
    dm: bool,
    bc: bool,
    of: bool,
    nf: bool,
}

impl CPU {
    fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            stack: [0; 256],
            ac: 0,
            idx: 0,
            idy: 0,
            cf: false,
            zf: false,
            id: false,
            dm: false,
            bc: false,
            of: false,
            nf: false,
        }
    }

    fn clear_stack(&mut self) {
        for i in 0..255 {
            self.stack[i] = 0;
        }
    }

    fn clear_flags(&mut self) {
        self.cf = false;
        self.zf = false;
        self.id = false;
        self.dm = false;
        self.bc = false;
        self.of = false;
        self.nf = false;
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pc: {}\nsp: {}\nac: {}\nidx: {}\nidy: {}\ncf: {}\nzf: {}\nid: {}\ndm: {}\nbc: {}\nof: {}\nnf: {}\n",
            self.pc,
            self.sp,
            self.ac,
            self.idx,
            self.idy,
            self.cf,
            self.zf,
            self.id,
            self.dm,
            self.bc,
            self.of,
            self.nf
        )
    }
}

fn main() {
    // eventually we need to error check this to actually make sure that we are being supplied
    // enough arguments
    let bin = fs::read(env::args().next_back().unwrap()).unwrap();
    let cpu = CPU::new();

    println!("{}", cpu);
}
