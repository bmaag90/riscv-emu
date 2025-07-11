use dram::{DRAM_SIZE, DRAM_BASE_ADDR, DramMemory};

pub struct BasicCpu {
    registers : [u32; 32],
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
        self.registers[2] = (DRAM_BASE_ADDR + DRAM_SIZE) as u32; // stack pointer
        self.pc = DRAM_BASE_ADDR;
    }

    pub fn print_registers(&self){
        println!("=== REGISTERS ===");
        println!("[0:3] {} {} {} {}", self.registers[0],self.registers[1],self.registers[2],self.registers[3]);
        println!("[4:7] {} {} {} {}", self.registers[4],self.registers[5],self.registers[6],self.registers[7]);
        println!("[8:11] {} {} {} {}", self.registers[8],self.registers[9],self.registers[10],self.registers[11]);
        println!("[12:15] {} {} {} {}", self.registers[12],self.registers[13],self.registers[14],self.registers[15]);
        println!("[16:19] {} {} {} {}", self.registers[16],self.registers[17],self.registers[18],self.registers[19]);
        println!("[20:23] {} {} {} {}", self.registers[20],self.registers[21],self.registers[22],self.registers[23]);
        println!("[24:27] {} {} {} {}", self.registers[24],self.registers[25],self.registers[26],self.registers[27]);
        println!("[28:31] {} {} {} {}", self.registers[28],self.registers[29],self.registers[30],self.registers[31]);
        println!("=================");
    }

    pub fn get_register(&self, idx: usize) -> u32{
        if idx > 32 {
            println!("Invalid register index {idx}");
            return 0
        }
        self.registers[idx]
    }

    pub fn set_register(&mut self, idx: usize, value: u32) {
        if idx > 32 {
            println!("Invalid register index {idx}");
            return 
        }
        println!("Setting register {idx} to {value}");
        self.registers[idx] = value;
    }
    //
    // Processing
    //
    pub fn fetch_instr(&mut self) -> u32{
        self.mem.dram_read(self.pc, 32) as u32 // read instruction at program counter, 4bytes
    }

    pub fn execute_instr(&mut self, instr: u32){
        let opcode: u32 = self.instr_opcode(instr);
        //let func3: u32 = self.instr_func3(instr);
        //let instr_funct7: u32 = self.instr_funct7(instr);

        match opcode {
            0b0010011 => self.execute_imm(instr),
            _ => println!("Instruction with opcode {opcode} not implemented yet"),
        }
    }
    //
    // Instruction decoding
    //
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

    //
    // Execute instructions
    //
    fn execute_imm(&mut self, instr: u32){
        let func3: u32 = self.instr_func3(instr);
        let rd: u32 = self.instr_rd(instr);
        let rs1: u32 = self.instr_rs1(instr);
        let imm: u32 = self.instr_imm_i(instr);
        println!("func3: {func3} - rd: {rd} - rs1: {rs1} - imm: {imm}");
        match func3 {
            0b000 => self.set_register(rd as usize, self.get_register(rs1 as usize) + imm), // addi
            _ => println!("Function (I-Type) with code func3 {func3} not implemented yet")
        }

    }

}