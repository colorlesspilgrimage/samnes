use std::env;
use std::fs;

mod cpu;
mod decoder;
mod ppu;
mod apu; // We can keep the module, even if it's unused for now

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a ROM file path.");
    let rom_bytes = fs::read(file_path).expect("Failed to read ROM file.");

    // --- Load the ROM ---
    if rom_bytes.len() < 16 || &rom_bytes[0..4] != b"NES\x1A" {
        panic!("Invalid iNES header.");
    }
    let prg_rom_size = rom_bytes[4] as usize * 16384;

    if prg_rom_size > 32768 {
        let mapper_num = (rom_bytes[7] & 0xF0) | (rom_bytes[6] >> 4);
        println!("Error: ROM requires Mapper {}, which is not supported.", mapper_num);
        return;
    }

    let prg_rom_start = 16;
    let prg_rom_end = prg_rom_start + prg_rom_size;
    let prg_rom = &rom_bytes[prg_rom_start..prg_rom_end];

    let mut cpu = cpu::CPU::new();
    cpu.load(prg_rom, 0x8000);
    if prg_rom_size == 16384 { // NROM-128
        cpu.load(prg_rom, 0xC000);
    }

    // --- Run the CPU ---
    cpu.reset();

    let mut total_cycles: u64 = 0;
    const MAX_CYCLES: u64 = 20_000_000;
    println!("Starting CPU execution for {} cycles...", MAX_CYCLES);

    loop {
        let cycles = cpu.step();
        total_cycles += cycles as u64;

        if cycles == 0 { // JAM
            println!("CPU halted after {} cycles.", total_cycles);
            break;
        }

        if total_cycles > MAX_CYCLES {
            println!("Emulation finished after {} cycles.", total_cycles);
            break;
        }
    }
    println!("\nFinal CPU State:");
    println!("{}", cpu);
}