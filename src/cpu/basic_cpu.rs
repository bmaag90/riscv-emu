use crate::memory::dram::{DRAM_SIZE, DRAM_BASE_ADDR, DramMemory};

pub const REGISTERS_COUNT: usize = 32;
pub const CSR_COUNT: usize = 4096; // Maximum number of CSRs in RV64
// RV64i
pub type TReg = u64;
pub type TInstr = u32;
pub type TImm = u64; // immediate value

pub struct BasicCpu {
    registers : [TReg; REGISTERS_COUNT], // General-purpose registers
    pc : TReg, // Program Counter
    pub mem : DramMemory, // Memory interface
    csr : [TReg; CSR_COUNT], // CSR registers
}

impl BasicCpu {
    
    pub fn new() -> BasicCpu{
        BasicCpu {
            registers: [0; REGISTERS_COUNT],
            pc: 0x0,
            mem: DramMemory {
                mem: vec![0; DRAM_SIZE]
            },
            csr: [0; CSR_COUNT] 
        }
    }   

    pub fn init(&mut self){
        self.registers[0] = 0x0; // const. zero
        self.registers[2] = (DRAM_BASE_ADDR + DRAM_SIZE) as TReg; // stack pointer
        self.pc = DRAM_BASE_ADDR as TReg;
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

    pub fn get_register(&self, idx: usize) -> TReg{
        if idx > REGISTERS_COUNT {
            println!("Invalid register index {idx}");
            return 0
        }
        self.registers[idx]
    }

    pub fn set_register(&mut self, idx: usize, value: TReg) {
        if idx > REGISTERS_COUNT {
            println!("Invalid register index {idx}");
            return 
        }
        println!("Setting register {idx} to {value}");
        self.registers[idx] = value;
    }

    pub fn get_pc(&self) -> TReg {
        self.pc
    }

    pub fn set_pc(&mut self, pc: TReg) {
        println!("Setting program counter to {pc}");
        self.pc = pc;
    }

    pub fn get_csr(&self, idx: usize) -> TReg {
        if idx >= CSR_COUNT {
            println!("Invalid CSR index {idx}");
            return 0;
        }
        self.csr[idx]
    }

    pub fn set_csr(&mut self, idx: usize, value: TReg) {
        if idx >= CSR_COUNT {
            println!("Invalid CSR index {idx}");
            return;
        }
        println!("Setting CSR {idx} to {value}");
        self.csr[idx] = value;
    }
    //
    // Processing
    //
    pub fn fetch_instr(&mut self) -> TInstr{
        self.mem.dram_read(self.get_pc() as usize, 32) as TInstr // read instruction at program counter, 4bytes
    }

    pub fn execute_instr(&mut self, instr: TInstr){
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
            0b0000011 => self.execute_load(instr),
            0b0100011 => self.execute_store(instr),
            0b0110011 => self.execute_r_type(instr),
            0b0001111 => self.execute_fence(instr),
            0b1110011 => self.execute_system_csr(instr), // SYSTEM instruction, e.g. ECALL, EBREAK
            _ => panic!("Instruction with opcode {opcode} not implemented yet"),
        }
    }
    //
    // Instruction decoding
    //
    pub fn instr_opcode(&self, instr: TInstr) -> TInstr{ 
        // cpu operation code
        instr & 0x7F // 0b0111 1111 -> bits[0:7]
    }

    pub fn instr_rd(&self, instr: TInstr) -> TInstr{ 
        // destination register
        (instr >> 7) & 0x1f // 0b0001 1111 -> bits[7:11]
    }

    pub fn instr_func3(&self, instr: TInstr) -> TInstr {
        (instr >> 12) & 0x07 // 0b0111 -> bits[12:14]
    }

    pub fn instr_rs1(&self, instr: TInstr) -> TInstr {
        (instr >> 15) & 0x1f // 0b0111 -> bits[15:19]
    }

    pub fn instr_rs2_shamt(&self, instr: TInstr) -> TInstr {
        (instr >> 20) & 0x1f // 0b0111 -> bits[20:24]
    }

    pub fn instr_funct7(&self, instr: TInstr) -> TInstr {
        (instr >> 25) & 0x7F // 0b0111 -> bits[25:31]
    }

    pub fn instr_imm_i(&self, instr: TInstr) -> TInstr {
        (instr >> 20) & 0xfff
    }
   
    pub fn instr_imm_s(&self, instr: TInstr) -> TInstr {
        // bits[0:4]              0b0111 -> bits[25:31]
        ((instr >> 7) & 0x1F) | ((instr & 0xfe000000) >> 20) 
    }

    pub fn instr_imm_u(&self, instr: TInstr) -> TInstr {
        instr & 0xfffff000 // NOTE: NOT bit shifted, lower 12 bits are already filled with zeros for LUI (& auipc) instr.
    }

    pub fn instr_imm_b(&self, instr: TInstr) -> TInstr {
        ((instr & 0x80000000) >> 19) | ((instr & 0x80) << 4) | ((instr >> 20) & 0x7e0) | ((instr >> 7) & 0x1e)
    }

    pub fn instr_imm_j(&self, instr: TInstr) -> TInstr {
        ((instr & 0x80000000) >> 11) | (instr & 0xff000) | ((instr >> 9) & 0x800) | ((instr >> 20) & 0x7fe)
    }

    pub fn instr_csr_addr(&self, instr: TInstr) -> TInstr {
        // CSR address is in bits [20:31] of the instruction
        (instr >> 20) & 0xfff // bits[20:31]
    }
    //
    // Execute instructions
    //
    pub fn execute_imm(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rd: TInstr = self.instr_rd(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let imm: TImm = self.instr_imm_i(instr) as i32 as i64 as u64; // sign-extend immediate value
        let rs2_shamt: TInstr = self.instr_rs2_shamt(instr);
        let func7: TInstr = self.instr_funct7(instr);
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
            0b000 => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_add(imm)), // addi
            0b001 => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_shl(rs2_shamt)), // slli
            0b010 => self.set_register(rd as usize, if (self.get_register(rs1 as usize) as i64) < (imm as i64) { 1 } else { 0 }), // slti
            0b011 => self.set_register(rd as usize, if self.get_register(rs1 as usize) < imm { 1 } else { 0 }), // sltiu
            0b100 => self.set_register(rd as usize, self.get_register(rs1 as usize) ^ imm), // xori
            0b101 => match func7 {
                0b0000000 => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_shr(rs2_shamt)), // srli - SRLI is a logical right shift (zeros are shifted into the upper bits).
                0b0100000 => self.set_register(rd as usize, ((self.get_register(rs1 as usize) as i64).wrapping_shr(rs2_shamt)) as TReg), // srai - SRAI is an arithmetic right shift (the original sign bit is copied into the vacated upper bits).
                _ => panic!("Function (I-Type) with code func3 {func3} AND func7 {func7} not found")
            },
            0b110 => self.set_register(rd as usize, self.get_register(rs1 as usize) | imm), // ori
            0b111 => self.set_register(rd as usize, self.get_register(rs1 as usize) & imm), // andi
            _ => panic!("Function (I-Type) with code func3 {func3} not found")
        }
    }

    pub fn execute_lui(&mut self, instr: TInstr){
        let rd: TInstr = self.instr_rd(instr);
        let imm: TImm = self.instr_imm_u(instr) as i32 as i64 as u64; // sign-extend immediate value
        println!("[Instruction] opcode (0b0110111): rd: {rd} - imm: {imm}");
        // imm[31:12] rd 0110111 LUI
        self.set_register(rd as usize, imm);
    }

    pub fn execute_auipc(&mut self, instr: TInstr){
        let rd: TInstr = self.instr_rd(instr);
        let imm: TImm = self.instr_imm_u(instr) as i32 as i64 as u64; // sign-extend immediate value
        let pc: TReg = self.get_pc();
        println!("[Instruction] opcode (0b0010111): rd: {rd} - imm: {imm} (- pc: {pc})");
        // imm[31:12] rd 0010111 AUIPC
        self.set_register(rd as usize, pc.wrapping_add(imm)); // add immediate value to current pc
    }

    pub fn execute_jal(&mut self, instr: TInstr){
        let rd: TInstr = self.instr_rd(instr);
        let imm: TImm = self.instr_imm_j(instr) as i32 as i64 as u64; // sign-extend immediate value
        let pc: TReg = self.get_pc();
        println!("[Instruction] opcode (0b1101111): rd: {rd} - imm: {imm} (- pc: {pc})");
        // imm[20] imm[10:1] imm[11] imm[19:12] rd 1101111 JAL
        self.set_register(rd as usize, pc.wrapping_add(4)); // store return address
        let new_pc: TReg = pc.wrapping_add(imm); // calculate new pc
        self.set_pc(new_pc); // jump to target address
    }

    pub fn execute_jalr(&mut self, instr: TInstr){
        let rd: TInstr = self.instr_rd(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let imm: TImm = self.instr_imm_i(instr) as i32 as i64 as u64; // sign-extend immediate value;
        let pc: TReg = self.get_pc();
        println!("[Instruction] opcode (0b1100111): rd: {rd} - rs1: {rs1} - imm: {imm} (- pc: {pc})");
        // imm[11:0] rs1 000 rd 1100111 JALR
        self.set_register(rd as usize, pc.wrapping_add(4)); // store return address
        self.set_pc(self.get_register(rs1 as usize).wrapping_add(imm) as TReg & !1); // jump to target address (clear LSB)
    }

    pub fn execute_branch(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let rs2: TInstr = self.instr_rs2_shamt(instr);
        let imm: TImm = self.instr_imm_b(instr) as i32 as i64 as u64; // sign-extend immediate value
        let pc: TReg = self.get_pc();
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
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));
                }
            },
            0b001 => { // BNE
                if self.get_register(rs1 as usize) != self.get_register(rs2 as usize) {
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));    
                }
            },
            0b100 => { // BLT
                if (self.get_register(rs1 as usize) as i32) < (self.get_register(rs2 as usize) as i32) {
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));    
                }
            },
            0b101 => { // BGE
                if (self.get_register(rs1 as usize) as i32) >= (self.get_register(rs2 as usize) as i32) {
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));    
                }
            },
            0b110 => { // BLTU
                if self.get_register(rs1 as usize) < self.get_register(rs2 as usize) {
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));
                }
            },
            0b111 => { // BGEU
                if self.get_register(rs1 as usize) >= self.get_register(rs2 as usize) {
                    self.set_pc(pc.wrapping_add(imm));
                } else {
                    self.set_pc(pc.wrapping_add(4));
                }
            },
            _ => println!("Function (B-Type) with code func3 {func3} not found")
        }   
    }

    pub fn execute_load(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rd: TInstr = self.instr_rd(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let imm: TImm = self.instr_imm_i(instr) as i32 as i64 as u64; // sign-extend immediate value
        println!("[Instruction] opcode (0b0000011): func3: {func3} - rd: {rd} - rs1: {rs1} - imm: {imm}");
        /*
        imm[11:0] rs1 000 rd 0000011 LB 
        imm[11:0] rs1 001 rd 0000011 LH 
        imm[11:0] rs1 010 rd 0000011 LW 
        imm[11:0] rs1 100 rd 0000011 LBU 
        imm[11:0] rs1 101 rd 0000011 LHU
        */
        let target_addr: usize = (self.get_register(rs1 as usize).wrapping_add(imm)) as usize;
        if target_addr >= DRAM_BASE_ADDR && target_addr < (DRAM_BASE_ADDR + DRAM_SIZE) {
            println!("Reading from DRAM at address {target_addr}");
        } else {
            println!("Attempt to read from invalid DRAM address {target_addr}");
            return;
        }
        match func3 {
            0b000 => {
                let val = self.mem.dram_read(target_addr, 8); // LB
                self.set_register(rd as usize, val as i8 as i64 as TReg);
            },  
            0b001 => {
                let val = self.mem.dram_read(target_addr, 16); // LH
                self.set_register(rd as usize, val as i16 as i64 as TReg);
            },
            0b010 => {
                let val = self.mem.dram_read(target_addr, 32); // LW
                self.set_register(rd as usize, val as i32 as i64 as TReg);
            },
            0b100 => {
                let val = self.mem.dram_read(target_addr, 8); // LBU
                self.set_register(rd as usize, val as u8 as TReg);
            },
            0b101 => {
                let val = self.mem.dram_read(target_addr, 16); // LHU
                self.set_register(rd as usize, val as u16 as TReg);
            },
            _ => println!("Function (Load-Type) with code func3 {func3} not found")
        }
    }

    pub fn execute_store(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let rs2: TInstr = self.instr_rs2_shamt(instr);
        let imm: TImm = self.instr_imm_s(instr) as i32 as i64 as u64; // sign-extend immediate value
        println!("[Instruction] opcode (0b0100011): func3: {func3} - rs1: {rs1} - rs2: {rs2} - imm: {imm}");
        /*
        imm[11:5] rs2 rs1 000 imm[4:0] 0100011 SB 
        imm[11:5] rs2 rs1 001 imm[4:0] 0100011 SH 
        imm[11:5] rs2 rs1 010 imm[4:0] 0100011 SW
        */
        let target_addr: usize = (self.get_register(rs1 as usize).wrapping_add(imm)) as usize;
        if target_addr >= DRAM_BASE_ADDR && target_addr < (DRAM_BASE_ADDR + DRAM_SIZE) {
            println!("Writing to DRAM at address {target_addr}");
        } else {
            println!("Attempt to write to invalid DRAM address {target_addr}");
            return;
        }
        match func3 {
            0b000 => {
                let val = self.get_register(rs2 as usize) as i8 as u64; // SB
                self.mem.dram_write(target_addr, 8, val);
            },
            0b001 => {
                let val = self.get_register(rs2 as usize) as i16 as u64; // SH
                self.mem.dram_write(target_addr, 16, val);
            },
            0b010 => {
                let val = self.get_register(rs2 as usize) as i32 as u64; // SW
                self.mem.dram_write(target_addr, 32, val);
            },
            _ => println!("Function (Store-Type) with code func3 {func3} not found")
        }
    }

    pub fn execute_r_type(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rd: TInstr = self.instr_rd(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let rs2: TInstr = self.instr_rs2_shamt(instr);
        let func7: TInstr = self.instr_funct7(instr);
        println!("[Instruction] opcode (0b0110011): func3: {func3} - rd: {rd} - rs1: {rs1} - rs2: {rs2} - func7: {func7}");
        /*
        0000000 rs2 rs1 000 rd 0110011 ADD 
        0100000 rs2 rs1 000 rd 0110011 SUB 
        0000000 rs2 rs1 001 rd 0110011 SLL 
        0000000 rs2 rs1 010 rd 0110011 SLT 
        0000000 rs2 rs1 011 rd 0110011 SLTU 
        0000000 rs2 rs1 100 rd 0110011 XOR 
        0000000 rs2 rs1 101 rd 0110011 SRL 
        0100000 rs2 rs1 101 rd 0110011 SRA 
        0000000 rs2 rs1 110 rd 0110011 OR 
        0000000 rs2 rs1 111 rd 0110011 AND
        */
        match (func3, func7) {
            (0b000, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_add(self.get_register(rs2 as usize))), // add
            (0b000, 0b0100000) => self.set_register(rd as usize, ((self.get_register(rs1 as usize) as i64).wrapping_sub(self.get_register(rs2 as usize) as i64)) as TReg), // sub
            (0b001, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_shl(self.get_register(rs2 as usize) as u32)), // sll
            (0b010, 0b0000000) => self.set_register(rd as usize, if (self.get_register(rs1 as usize) as i64) < (self.get_register(rs2 as usize) as i64) { 1 } else { 0 }), // slt
            (0b011, 0b0000000) => self.set_register(rd as usize, if self.get_register(rs1 as usize) < self.get_register(rs2 as usize) { 1 } else { 0 }), // sltu
            (0b100, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize) ^ self.get_register(rs2 as usize)), // xor
            (0b101, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize).wrapping_shr(self.get_register(rs2 as usize) as u32)), // srl
            (0b101, 0b0100000) => self.set_register(rd as usize, ((self.get_register(rs1 as usize) as i64).wrapping_shr(self.get_register(rs2 as usize) as u32)) as TReg), // sra
            (0b110, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize) | self.get_register(rs2 as usize)), // or
            (0b111, 0b0000000) => self.set_register(rd as usize, self.get_register(rs1 as usize) & self.get_register(rs2 as usize)), // and
            _ => println!("Function (R-Type) with code func3 {func3} AND func7 {func7} not found")
        }   
    }

    pub fn execute_fence(&mut self, _instr: TInstr){
        // FENCE instruction is used to order memory operations
        // It does not change the state of the CPU or registers
        println!("[Instruction] opcode (0b0001111): FENCE instruction executed");
        // No operation needed for this implementation
    }

    pub fn execute_system_csr(&mut self, instr: TInstr){
        let func3: TInstr = self.instr_func3(instr);
        let rd: TInstr = self.instr_rd(instr);
        let rs1: TInstr = self.instr_rs1(instr);
        let csr_addr: TInstr = self.instr_csr_addr(instr);
        match func3 {
            0b000 => println!("[Instruction] opcode (0b1110011): ECALL/EBREAK"), // ECALL or EBREAK instruction
            0b001 => {
                /*  
                CSRRW - Read CSR and write to register
                CSRRW reads the old value of the CSR, zero-extends the value to XLEN bits, then writes it to integer register rd. 
                The initial value in rs1 is written to the CSR
                */
                let csr_value = self.get_csr(csr_addr as usize);
                self.set_register(rd as usize, csr_value);
                if rs1 != 0 {
                    // Write value from register to CSR
                    self.set_csr(csr_addr as usize, self.get_register(rs1 as usize));
                }
                println!("[Instruction] opcode (0b1110011): CSRRW - CSR: {csr_addr} - rd: {rd} - rs1: {rs1}");
            },
            0b010 => {
                /*
                CSRRS - Read CSR and set bits from register
                reads the value of the CSR, zeroextends the value to XLEN bits, and writes it to integer register rd. 
                The initial value in integer register rs1 is treated as a bit mask that specifies bit positions to be set in the CSR.
                Any bit that is high in rs1 will cause the corresponding bit to be set in the CSR, if that CSR bit is writable.
                 */
                let csr_value = self.get_csr(csr_addr as usize);
                self.set_register(rd as usize, csr_value);
                if rs1 != 0 {
                    // Set bits in CSR from register
                    self.set_csr(csr_addr as usize, csr_value | self.get_register(rs1 as usize));
                }
                println!("[Instruction] opcode (0b1110011): CSRRS - CSR: {csr_addr} - rd: {rd} - rs1: {rs1}");
            },
            0b011 => {
                /*
                CSRRC - Read CSR and clear bits from register
                reads the value of the CSR, zeroextends the value to XLEN bits, and writes it to integer register rd. 
                The initial value in integer register rs1 is treated as a bit mask that specifies bit positions to be cleared in the CSR. 
                Any bit that is high in rs1 will cause the corresponding bit to be cleared in the CSR, if that CSR bit is writable
                 */
                let csr_value = self.get_csr(csr_addr as usize);
                self.set_register(rd as usize, csr_value);
                if rs1 != 0 {
                    // Clear bits in CSR from register
                    self.set_csr(csr_addr as usize, csr_value & !self.get_register(rs1 as usize));
                }
                println!("[Instruction] opcode (0b1110011): CSRRC - CSR: {csr_addr} - rd: {rd} - rs1: {rs1}");
            },
            0b101 => {
                /*
                CSRRWI - Read CSR and write immediate value
                CSRRWI reads the old value of the CSR, zero-extends the value to XLEN bits, then writes it to integer register rd. 
                The immediate value is written to the CSR.
                 */
                let csr_value = self.get_csr(csr_addr as usize);
                if rd != 0 {
                    self.set_register(rd as usize, csr_value);
                    self.set_csr(csr_addr as usize, rs1 as TReg); // rs1 is used as immediate value
                } else {
                    println!("Warning: CSRRWI instruction with rd = 0, no value will be stored in register");
                }
                println!("[Instruction] opcode (0b1110011): CSRRWI - CSR: {csr_addr} - rd: {rd} - imm: {rs1}");
            },
            0b110 => {
                /*                
                CSRRSI - Read CSR and set bits from immediate value
                reads the value of the CSR, zero-extends the value to XLEN bits, and writes it to integer register rd.
                The immediate value is treated as a bit mask that specifies bit positions to be set in the CSR.
                Any bit that is high in the immediate value will cause the corresponding bit to be set in the CSR, if that CSR bit is writable.
                */
                let csr_value = self.get_csr(csr_addr as usize);
                self.set_register(rd as usize, csr_value);
                if rs1 != 0 {
                    // Set bits in CSR from immediate value
                    self.set_csr(csr_addr as usize, csr_value | rs1 as TReg);
                }
                println!("[Instruction] opcode (0b1110011): CSRRSI - CSR: {csr_addr} - rd: {rd} - imm: {rs1}");
            },
            0b111 => {
                /*                
                CSRRCI - Read CSR and clear bits from immediate value
                reads the value of the CSR, zero-extends the value to XLEN bits, and writes it to integer register rd.  
                The immediate value is treated as a bit mask that specifies bit positions to be cleared in the CSR.
                Any bit that is high in the immediate value will cause the corresponding bit to be cleared in the CSR, if that CSR bit is writable.
                */
                let csr_value = self.get_csr(csr_addr as usize);
                self.set_register(rd as usize, csr_value);
                if rs1 != 0 {
                    // Clear bits in CSR from immediate value
                    self.set_csr(csr_addr as usize, csr_value & !(rs1 as TReg));
                }
                println!("[Instruction] opcode (0b1110011): CSRRCI - CSR: {csr_addr} - rd: {rd} - imm: {rs1}");
            },
            _ => println!("Function (System-CSR) with code func3 {func3} not found")
        }
    }
}