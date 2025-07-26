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

    info!("INIT - Loaded binary into DRAM memory.");
    
    info!("INIT - Initializing CPU...");
    cpu.init();
    info!("INIT - Current registers:");
    cpu.print_registers();
    info!("INIT - Current PC: {:#x}", cpu.get_pc());
    info!("START - Starting execution...");
    loop {

        let current_instruction = cpu.fetch_instr();
        let current_pc = cpu.get_pc();

        info!("MAIN LOOP - PC: {:#x}, Instruction: {:#x}", current_pc, current_instruction);

        match cpu.execute_instr(current_instruction) {
            Ok(_) => {},
            Err(err) => {
                println!("MAIN LOOP - Error executing instruction at PC {:#x}: {}", current_pc, err);
                break
            },
        };
 

        cpu.set_pc(cpu.get_pc() + 4);

        if cpu.get_pc() >= (dram::DRAM_BASE_ADDR + dram::DRAM_SIZE) as u64 {
            warn!("MAIN LOOP - Reached end of DRAM memory.");
            break;
        }

        if cpu.get_pc() == 0 {
            info!("MAIN LOOP - Program terminated.");
            break;
        }
    }
    info!("DONE - Final CPU state:");
    cpu.print_registers();
    info!("DONE - Final PC: {:#x}", cpu.get_pc());
    info!("DONE - Emulation finished.");

}