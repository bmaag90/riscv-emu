mod memory {
    pub mod dram;
}
use memory::dram;
mod cpu {
    pub mod basic_cpu;
}
use cpu::basic_cpu;

fn main() {
    println!("Hello there!");

    let mut my_dram_memory = dram::DramMemory {
       mem: vec![0; dram::DRAM_SIZE]
    };
    println!("=====\n DRAM\n=====");
    println!("Init DRAM");

    println!("Writing to DRAM");
    my_dram_memory.dram_write(
        0x80000004, 4*8, 0xAB
    );
    println!("Reading from DRAM");
    let r_val = my_dram_memory.dram_read(0x80000004, 4*8);

    if r_val == 0xAB {
        println!("W/R succesful");
    } else {
        println!("Error W/R - got value = {r_val}");
    }

    println!("=====\n Instructions\n=====");

    let mut my_cpu = basic_cpu::BasicCpu::new();

    
}