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
        println!("Error W/R - got value = 0x{:08x}", r_val);
    }

    println!("Writing to DRAM");
    my_dram_memory.dram_write(
        0x80000004, 4*8, 0xA08113
    );
    println!("Reading from DRAM");
    let r_val = my_dram_memory.dram_read(0x80000004, 4*8);

    if r_val == 0xA08113 {
        println!("W/R succesful");
    } else {
        println!("Error W/R - got value = 0x{:08x}", r_val);
    }

    println!("=====\n Instructions\n=====");

    let mut my_cpu = basic_cpu::BasicCpu::new();

    my_cpu.init();

    
    my_cpu.mem.dram_write(dram::DRAM_BASE_ADDR, 4*8, 0b00000000101000101000000110010011);
    let instr: u64 = my_cpu.mem.dram_read(dram::DRAM_BASE_ADDR, 4*8);
    my_cpu.print_registers();
    println!("Added instruction to PC addr - instr = 0x{:08x}", instr);
    my_cpu.set_register(5, 7);
    let f_instr: u32 = my_cpu.fetch_instr();
    println!("Fetched instruction: 0x{:08x}", f_instr);

    my_cpu.execute_instr(f_instr);
    println!("Executed instruction: 0x{:08x}", f_instr);
    my_cpu.print_registers();
    let content_r2 = my_cpu.get_register(2);
    let content_r1 = my_cpu.get_register(1);

    println!("Registers r1: {content_r1} - r2: {content_r2}");
}