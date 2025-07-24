use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

mod memory {
    pub mod dram;
}
use memory::dram;
mod cpu {
    pub mod basic_cpu;
}
use cpu::basic_cpu;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: main <binary_filename>");
    }
    let mut file = File::open(&args[1])
        .expect("Failed to open binary file");
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)
        .expect("Failed to read binary file");

    let mut cpu = basic_cpu::BasicCpu::new();

    for (i, byte) in binary.iter().enumerate() {
        cpu.mem.dram_write(dram::DRAM_BASE_ADDR + i, 8, (*byte).into());
    }

    println!("Loaded binary into DRAM memory.");
    
    println!("Initializing CPU...");
    cpu.init();
    println!("Current registers:");
    cpu.print_registers();
    println!("Current PC: {:#x}", cpu.get_pc());
    println!("Starting execution...");
    loop {

        let current_instruction = cpu.fetch_instr();
        let current_pc = cpu.get_pc();

        println!("PC: {:#x}, Instruction: {:#x}", current_pc, current_instruction);

        cpu.execute_instr(current_instruction); 

        cpu.set_pc(cpu.get_pc() + 4);

        if cpu.get_pc() >= (dram::DRAM_BASE_ADDR + dram::DRAM_SIZE) as u64 {
            println!("Reached end of DRAM memory.");
            break;
        }

        if cpu.get_pc() == 0 {
            println!("Program terminated.");
            break;
        }
    }
}