use dram::{DRAM_SIZE, DRAM_BASE_ADDR, DramMemory};

pub struct BasicCpu {
    registers : [usize; 32],
    pc : usize,
    pub mem : DramMemory
}

impl BasicCpu {
    
    pub fn new() -> BasicCpu{
        BasicCpu {
            registers: [0; 32],
            pc: 0x0,
            mem: DramMemory {
                mem: vec![0; DRAM_SIZE]
            }
        }
    }

    pub fn init(&mut self){
        self.registers[0] = 0x0; // const. zero
        self.registers[2] = DRAM_BASE_ADDR + DRAM_SIZE; // stack pointer
        self.pc = DRAM_BASE_ADDR;
    }
    
    pub fn fetch_instr(&mut self) -> u32{
        self.mem.dram_read(self.pc, 32) as u32 // read instruction at program counter, 4bytes
    }

    pub fn execute_instr(&self, instr: u32){
        let opcode: u32 = self.instr_opcode(instr);
        let func3: u32 = self.instr_func3(instr);
        let instr_funct7: u32 = self.instr_funct7(instr);
    }

    fn instr_opcode(&self, instr: u32) -> u32{ 
        // cpu operation code
        instr & 0x7F // 0b0111 1111 -> bits[0:7]
    }

    fn instr_rd(&self, instr: u32) -> u32{ 
        // destination register
        (instr >> 7) & 0x1f // 0b0001 1111 -> bits[7:11]
    }

    fn instr_func3(&self, instr: u32) -> u32 {
        (instr >> 12) & 0x07 // 0b0111 -> bits[12:14]
    }

    fn instr_rs1(&self, instr: u32) -> u32 {
        (instr >> 15) & 0x07 // 0b0111 -> bits[15:19]
    }

    fn instr_rs2(&self, instr: u32) -> u32 {
        (instr >> 20) & 0x07 // 0b0111 -> bits[20:24]
    }

    fn instr_funct7(&self, instr: u32) -> u32 {
        (instr >> 25) & 0x7F // 0b0111 -> bits[25:31]
    }

    fn instr_imm_i(&self, instr: u32) -> u32 {
        (instr >> 20) & 0xfff
    }
   
    fn instr_imm_s(&self, instr: u32) -> u32 {
        // bits[0:4]              0b0111 -> bits[25:31]
        ((instr >> 7) & 0x1F) | ((instr & 0xfe000000) >> 20) 
    }

    fn instr_imm_u(&self, instr: u32) -> u32 {
        instr & 0xfffff999
    }

    fn instr_imm_b(&self, instr: u32) -> u32 {
        ((instr & 0x80000000) >> 19) | ((instr & 0x80) << 4) | ((instr >> 20) & 0x7e0) | ((instr >> 7) & 0x1e)
    }

    fn instr_imm_j(&self, instr: u32) -> u32 {
        ((instr & 0x80000000) >> 11) | (instr & 0xff000) | ((instr >> 9) & 0x800) | ((instr >> 20) & 0x7fe)
    }

}