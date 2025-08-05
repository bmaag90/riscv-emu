use std::fs::File;
use std::io::prelude::*;
use std::env;
use log::{info, warn};
use env_logger;

mod memory {
    pub mod dram;
}
use memory::dram;
mod cpu {
    pub mod basic_cpu;
}
use cpu::basic_cpu;

fn main() {
    env_logger::init();
    info!("Starting RISC-V Emulator...");
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

    info!("Init - Loaded binary into DRAM memory.");
    
    info!("Init - Initializing CPU...");
    cpu.init();
    info!("Init - Current registers:");
    cpu.print_registers();
    info!("Init - Current PC: {:#x}", cpu.get_pc());
    info!("Init - setting ra to 0x0");
    cpu.set_register(1, 0x0); // ra = register x1
    info!("Init - Starting execution...");
    loop {

        let current_instruction = cpu.fetch_instr();
        let current_pc = cpu.get_pc();

        info!("PC: {:#x}, Instruction: {:#x}", current_pc, current_instruction);

        cpu.set_pc(cpu.get_pc() + 4);

        match cpu.execute_instr(current_instruction) {
            Ok(_) => {},
            Err(err) => {
                println!("Error executing instruction at PC {:#x}: {}", current_pc, err);
                break
            },
        };

        if cpu.get_pc() >= (dram::DRAM_BASE_ADDR + dram::DRAM_SIZE) as u64 {
            warn!("Reached end of DRAM memory.");
            break;
        }

        if cpu.get_pc() == 0 {
            info!("Program terminated.");
            break;
        }
    }
    info!("Done - Final CPU state:");
    cpu.print_registers();
    info!("Done - Final PC: {:#x}", cpu.get_pc());
    info!("Done - Emulation finished.");

}