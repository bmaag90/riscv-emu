mod memory;
use memory::dram;

fn main() {
    println!("Hello, world!");

    let mut my_dram_memory = dram::DramMemory {
       mem: vec![0; dram::DRAM_SIZE]
    };
    println!("Init DRAM");
    // let mut  my_dram_memory = Box::new(
    //     dram::DramMemory {
    //         mem: [0; dram::DRAM_SIZE]
    //     }
    // );
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
}