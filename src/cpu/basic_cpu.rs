use dram::{DRAM_SIZE, DRAM_BASE_ADDR, DramMemory};

struct BasicCpu {
    registers : [usize; 32],
    pc : usize,
    mem : DramMemory
}

impl BasicCpu {
 
    fn init(&mut self){
        self.registers[0] = 0x0; // const. zero
        self.registers[2] = DRAM_BASE_ADDR + DRAM_SIZE; // stack pointer
        self.pc = DRAM_BASE_ADDR;
    }
    
    fn fetch_instr(&mut self) -> u32{
        self.mem.dram_read(self.pc, 32) as u32 // read instruction at program counter, 4bytes
    }

    fn instr_opcode(instr: u32) -> u32{ 
        // cpu operation code
        instr & 0x7F // 0b0111 1111 -> bits[0:7]
    }

    fn instr_rd(instr: u32) -> u32{ 
        // destination register
        (instr >> 7) & 0x1f // 0b0001 1111 -> bits[7:11]
    }

    fn instr_func3(instr: u32) -> u32 {
        (instr >> 12) & 0x07 // 0b0111 -> bits[12:14]
    }

    fn instr_rs1(instr: u32) -> u32 {
        (instr >> 15) & 0x07 // 0b0111 -> bits[15:19]
    }

    fn instr_rs2(instr: u32) -> u32 {
        (instr >> 20) & 0x07 // 0b0111 -> bits[20:24]
    }

    fn instr_funct7(instr: u32) -> u32 {
        (instr >> 25) & 0x7F // 0b0111 -> bits[25:31]
    }

    fn instr_imm_i(instr: u32) -> u32 {
        (instr >> 20) & 0xfff
    }
   
}