use std::env;
use std::fs;

mod cpu;
mod decoder;

/*
* endian-ness: the 6502 is a little endian system. when we read bytecode, while the first byte will
* be the opcode, we must keep in mind that reading operands will then have to be flipped (i.e.
* 0x123456 would need to be read ultimately into our buffer as 56 34 12)
* */

fn main() {
    // eventually we need to error check this to actually make sure that we are being supplied
    // enough arguments
    let mut bin = fs::read(env::args().next_back().unwrap()).unwrap();
    let cpu = cpu::CPU::new();

    // we need to remember with decoding in the long run that some parts of the file 
    // will be audio data, graphical data, and a header section at the beginning. a lot of the
    // opcodes being decoded right now are not meant to be real instructions.
    let dec = decoder::decode(&mut bin);

    for i in dec {
        println!("{:?}", i);
    }
}
