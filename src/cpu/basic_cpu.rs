use crate::memory::dram::{DRAM_SIZE, DRAM_BASE_ADDR, DramMemory};

pub const REGISTERS_COUNT: usize = 32;
pub type TypeRegister = u32;

pub struct BasicCpu {
    registers : [u32; REGISTERS_COUNT],
    pc : TypeRegister,
    pub mem : DramMemory
}

impl BasicCpu {
    
    pub fn new() -> BasicCpu{
        BasicCpu {
            registers: [0; REGISTERS_COUNT],
            pc: 0x0,
            mem: DramMemory {
                mem: vec![0; DRAM_SIZE]
            }
        }
    }   

    pub fn init(&mut self){
        self.registers[0] = 0x0; // const. zero
        self.registers[2] = (DRAM_BASE_ADDR + DRAM_SIZE) as TypeRegister; // stack pointer
        self.pc = DRAM_BASE_ADDR as TypeRegister;
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

    pub fn get_register(&self, idx: usize) -> TypeRegister{
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

    pub fn get_pc(&self) -> TypeRegister {
        self.pc
    }

    pub fn set_pc(&mut self, pc: TypeRegister) {
        println!("Setting program counter to {pc}");
        self.pc = pc;
    }
    //
    // Processing
    //
    pub fn fetch_instr(&mut self) -> u32{
        self.mem.dram_read(self.pc.try_into().unwrap(), 32) as u32 // read instruction at program counter, 4bytes
    }

    pub fn execute_instr(&mut self, instr: u32){
        let opcode: u32 = self.instr_opcode(instr);
        //let func3: u32 = self.instr_func3(instr);
        //let instr_funct7: u32 = self.instr_funct7(instr);

        match opcode {
            0b0010011 => self.execute_imm(instr),
            0b0110111 => self.execute_lui(instr),
            0b0010111 => self.execute_auipc(instr),
            0b1101111 => self.execute_jal(instr),
            0b1100111 => self.execute_jalr(instr),
            0b1100011 => self.execute_branch(instr),
            _ => println!("Instruction with opcode {opcode} not implemented yet"),
        }
    }
    //
    // Instruction decoding
    //
    pub fn instr_opcode(&self, instr: u32) -> u32{ 
        // cpu operation code
        instr & 0x7F // 0b0111 1111 -> bits[0:7]
    }

    pub fn instr_rd(&self, instr: u32) -> u32{ 
        // destination register
        (instr >> 7) & 0x1f // 0b0001 1111 -> bits[7:11]
    }

    pub fn instr_func3(&self, instr: u32) -> u32 {
        (instr >> 12) & 0x07 // 0b0111 -> bits[12:14]
    }

    pub fn instr_rs1(&self, instr: u32) -> u32 {
        (instr >> 15) & 0x07 // 0b0111 -> bits[15:19]
    }

    pub fn instr_rs2_shamt(&self, instr: u32) -> u32 {
        (instr >> 20) & 0x07 // 0b0111 -> bits[20:24]
    }

    pub fn instr_funct7(&self, instr: u32) -> u32 {
        (instr >> 25) & 0x7F // 0b0111 -> bits[25:31]
    }

    pub fn instr_imm_i(&self, instr: u32) -> u32 {
        (instr >> 20) & 0xfff
    }
   
    pub fn instr_imm_s(&self, instr: u32) -> u32 {
        // bits[0:4]              0b0111 -> bits[25:31]
        ((instr >> 7) & 0x1F) | ((instr & 0xfe000000) >> 20) 
    }

    pub fn instr_imm_u(&self, instr: u32) -> u32 {
        instr & 0xfffff000 // NOTE: NOT bit shifted, lower 12 bits are already filled with zeros for LUI (& auipc) instr.
    }

    pub fn instr_imm_b(&self, instr: u32) -> u32 {
        ((instr & 0x80000000) >> 19) | ((instr & 0x80) << 4) | ((instr >> 20) & 0x7e0) | ((instr >> 7) & 0x1e)
    }

    pub fn instr_imm_j(&self, instr: u32) -> u32 {
        ((instr & 0x80000000) >> 11) | (instr & 0xff000) | ((instr >> 9) & 0x800) | ((instr >> 20) & 0x7fe)
    }

    //
    // Execute instructions
    //
    pub fn execute_imm(&mut self, instr: u32){
        let func3: u32 = self.instr_func3(instr);
        let rd: u32 = self.instr_rd(instr);
        let rs1: u32 = self.instr_rs1(instr);
        let imm: u32 = self.instr_imm_i(instr);
        let rs2_shamt: u32 = self.instr_rs2_shamt(instr);
        let func7: u32 = self.instr_funct7(instr);
        println!("[Instruction] opcode (0b0010011): func3: {func3} - rd: {rd} - rs1: {rs1} - imm: {imm}");
        /*
        imm[11:0] rs1 000 rd 0010011 ADDI 
        imm[11:0] rs1 010 rd 0010011 SLTI 
        imm[11:0] rs1 011 rd 0010011 SLTIU 
        imm[11:0] rs1 100 rd 0010011 XORI 
        imm[11:0] rs1 110 rd 0010011 ORI 
        imm[11:0] rs1 111 rd 0010011 ANDI
        0000000 shamt rs1 001 rd 0010011 SLLI 
        0000000 shamt rs1 101 rd 0010011 SRLI 
        0100000 shamt rs1 101 rd 0010011 SRAI
        */
        match func3 {
            0b000 => self.set_register(rd as usize, self.get_register(rs1 as usize) + imm), // addi
            0b001 => self.set_register(rd as usize, self.get_register(rs1 as usize) << rs2_shamt), // slli
            0b010 => self.set_register(rd as usize, if (self.get_register(rs1 as usize) as i32) < (imm as i32) { 1 } else { 0 }), // slti
            0b011 => self.set_register(rd as usize, if self.get_register(rs1 as usize) < imm { 1 } else { 0 }), // sltiu
            0b100 => self.set_register(rd as usize, self.get_register(rs1 as usize) ^ imm), // xori
            0b101 => match func7 {
                0b0000000 => self.set_register(rd as usize, self.get_register(rs1 as usize) >> rs2_shamt), // srli - SRLI is a logical right shift (zeros are shifted into the upper bits).
                0b0100000 => self.set_register(rd as usize, ((self.get_register(rs1 as usize) as i32) >> rs2_shamt) as u32), // srai - SRAI is an arithmetic right shift (the original sign bit is copied into the vacated upper bits).
                _ => println!("Function (I-Type) with code func3 {func3} AND func7 {func7} not found")
            },
            0b110 => self.set_register(rd as usize, self.get_register(rs1 as usize) | imm), // ori
            0b111 => self.set_register(rd as usize, self.get_register(rs1 as usize) & imm), // andi
            _ => println!("Function (I-Type) with code func3 {func3} not found")
        }
    }

    pub fn execute_lui(&mut self, instr: u32){
        let rd: u32 = self.instr_rd(instr);
        let imm: u32 = self.instr_imm_u(instr);
        println!("[Instruction] opcode (0b0110111): rd: {rd} - imm: {imm}");
        // imm[31:12] rd 0110111 LUI
        self.set_register(rd as usize, imm);
    }

    pub fn execute_auipc(&mut self, instr: u32){
        let rd: u32 = self.instr_rd(instr);
        let imm: u32 = self.instr_imm_u(instr);
        let pc: u32 = self.get_pc() as u32;
        println!("[Instruction] opcode (0b0010111): rd: {rd} - imm: {imm} (- pc: {pc})");
        // imm[31:12] rd 0010111 AUIPC
        self.set_register(rd as usize, pc + imm)
    }

    pub fn execute_jal(&mut self, instr: u32){
        let rd: u32 = self.instr_rd(instr);
        let imm: u32 = self.instr_imm_j(instr);
        let pc: u32 = self.get_pc() as u32;
        println!("[Instruction] opcode (0b1101111): rd: {rd} - imm: {imm} (- pc: {pc})");
        // imm[20] imm[10:1] imm[11] imm[19:12] rd 1101111 JAL
        self.set_register(rd as usize, pc + 4); // store return address
        self.set_pc((pc as i32 + imm as i32) as TypeRegister); // jump to target address
    }

    pub fn execute_jalr(&mut self, instr: u32){
        let rd: u32 = self.instr_rd(instr);
        let rs1: u32 = self.instr_rs1(instr);
        let imm: u32 = self.instr_imm_i(instr);
        let pc: u32 = self.get_pc() as u32;
        println!("[Instruction] opcode (0b1100111): rd: {rd} - rs1: {rs1} - imm: {imm} (- pc: {pc})");
        // imm[11:0] rs1 000 rd 1100111 JALR
        self.set_register(rd as usize, pc + 4); // store return address
        self.set_pc((self.get_register(rs1 as usize) + imm) as TypeRegister & !1); // jump to target address (clear LSB)
    }

    pub fn execute_branch(&mut self, instr: u32){
        let func3: u32 = self.instr_func3(instr);
        let rs1: u32 = self.instr_rs1(instr);
        let rs2: u32 = self.instr_rs2_shamt(instr);
        let imm: u32 = self.instr_imm_b(instr);
        let pc: u32 = self.get_pc() as u32;
        println!("[Instruction] opcode (0b1100011): func3: {func3} - rs1: {rs1} - rs2: {rs2} - imm: {imm} (- pc: {pc})");
        /*
        imm[12|10:5] rs2 rs1 000 imm[4:1|11] 1100011 BEQ 
        imm[12|10:5] rs2 rs1 001 imm[4:1|11] 1100011 BNE 
        imm[12|10:5] rs2 rs1 100 imm[4:1|11] 1100011 BLT 
        imm[12|10:5] rs2 rs1 101 imm[4:1|11] 1100011 BGE 
        imm[12|10:5] rs2 rs1 110 imm[4:1|11] 1100011 BLTU 
        imm[12|10:5] rs2 rs1 111 imm[4:1|11] 1100011 BGEU
        */
        match func3 {
            0b000 => { // BEQ
                if self.get_register(rs1 as usize) == self.get_register(rs2 as usize) {
                    self.set_pc((pc as i64 + imm as i64) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);
                }
            },
            0b001 => { // BNE
                if self.get_register(rs1 as usize) != self.get_register(rs2 as usize) {
                    self.set_pc((pc as i32 + imm as i32) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);    
                }
            },
            0b100 => { // BLT
                if (self.get_register(rs1 as usize) as i32) < (self.get_register(rs2 as usize) as i32) {
                    self.set_pc((pc as i32 + imm as i32) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);    
                }
            },
            0b101 => { // BGE
                if (self.get_register(rs1 as usize) as i32) >= (self.get_register(rs2 as usize) as i32) {
                    self.set_pc((pc as i32 + imm as i32) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);    
                }
            },
            0b110 => { // BLTU
                if self.get_register(rs1 as usize) < self.get_register(rs2 as usize) {
                    self.set_pc((pc as i32 + imm as i32) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);
                }
            },
            0b111 => { // BGEU
                if self.get_register(rs1 as usize) >= self.get_register(rs2 as usize) {
                    self.set_pc((pc as i32 + imm as i32) as TypeRegister);
                } else {
                    self.set_pc((pc + 4) as TypeRegister);
                }
            },
            _ => println!("Function (B-Type) with code func3 {func3} not found")
        }   
    }
}