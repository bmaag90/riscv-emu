use riscv_emu::cpu::basic_cpu::BasicCpu;
use riscv_emu::memory::dram::DRAM_BASE_ADDR;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Check if register x0 is zero (hardwired)
        assert_eq!(cpu.get_register(0), 0);
        
        // Check if PC is initialized to DRAM base address
        assert_eq!(cpu.get_pc(), DRAM_BASE_ADDR.try_into().unwrap());
    }

    #[test]
    fn test_register_operations() {
        let mut cpu = BasicCpu::new();
        
        // Test register write/read
        cpu.set_register(1, 42);
        assert_eq!(cpu.get_register(1), 42);
        
    }

    #[test]
    fn test_instruction_fetch() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Write test instruction to memory
        let test_instruction = 0x00A00093; // addi x1, x0, 10
        cpu.mem.dram_write(DRAM_BASE_ADDR, 32, test_instruction as u64);
        
        // Fetch and verify instruction
        let fetched = cpu.fetch_instr();
        assert_eq!(fetched, test_instruction);
    }

    #[test]
    fn test_instruction_decode() {
        let cpu = BasicCpu::new();
        
        // Test instruction: addi x2, x1, 5
        // Format: imm[11:0] rs1 000 rd 0010011
        let instruction = 0x00508113;
        
        assert_eq!(cpu.instr_opcode(instruction), 0b0010011);
        assert_eq!(cpu.instr_rd(instruction), 2);       // x2
        assert_eq!(cpu.instr_func3(instruction), 0);    // addi
        assert_eq!(cpu.instr_rs1(instruction), 1);      // x1
        assert_eq!(cpu.instr_imm_i(instruction), 5);    // immediate value
    }

    #[test]
    fn test_addi_execution() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Set initial value in source register
        cpu.set_register(1, 5);
        
        // Create ADDI instruction: add 3 to x1 and store in x2
        let instruction = 0x00308113; // addi x2, x1, 3
        
        cpu.execute_instr(instruction);
        
        // Check result: 5 + 3 = 8
        assert_eq!(cpu.get_register(2), 8);
    }

    #[test]
    fn test_shift_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test SLLI (Shift Left Logical Immediate)
        cpu.set_register(1, 0x1);
        let slli = 0x00109093;    // slli x1, x1, 1
        cpu.execute_instr(slli);
        assert_eq!(cpu.get_register(1), 0x2);

        // Test SRLI (Shift Right Logical Immediate)
        cpu.set_register(2, 0x8);
        let srli = 0x00115113;    // srli x2, x2, 1
        cpu.execute_instr(srli);
        assert_eq!(cpu.get_register(2), 0x4);

        // Test SRAI (Shift Right Arithmetic Immediate)
        cpu.set_register(3, 0xFFFFFFFFFFFFFFFF);  // -1 in two's complement
        let srai = 0x41f1d193;    // srai x3, x3, 31
        cpu.execute_instr(srai);
        assert_eq!(cpu.get_register(3), 0xFFFFFFFFFFFFFFFF);  // Should remain -1
    }

    #[test]
    fn test_set_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test SLTI (Set Less Than Immediate)
        cpu.set_register(1, 5);
        let slti = 0x00a0a093;    // slti x1, x1, 10
        cpu.execute_instr(slti);
        assert_eq!(cpu.get_register(1), 1);  // 5 < 10 -> true (1)

        // Test SLTIU (Set Less Than Immediate Unsigned)
        cpu.set_register(2, 10);
        let sltiu = 0x00513113;    // sltiu x2, x2, 5
        cpu.execute_instr(sltiu);
        assert_eq!(cpu.get_register(2), 0);  // 10 < 5 -> false (0)
    }

    #[test]
    fn test_logical_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test XORI
        cpu.set_register(1, 0xFF);
        let xori = 0x00f0c093;    // xori x1, x1, 15
        cpu.execute_instr(xori);
        assert_eq!(cpu.get_register(1), 0xF0);

        // Test ORI
        cpu.set_register(2, 0xF0);
        let ori = 0x00f16113;     // ori x2, x2, 15
        cpu.execute_instr(ori);
        assert_eq!(cpu.get_register(2), 0xFF);

        // Test ANDI
        cpu.set_register(3, 0xFF);
        let andi = 0x00f1f193;    // andi x3, x3, 15
        cpu.execute_instr(andi);
        assert_eq!(cpu.get_register(3), 0x0F);
    }

    #[test]
    fn test_lui_instruction() {
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test LUI (Load Upper Immediate)
        let lui = 0x123450b7;     // lui x1, 0x12345
        cpu.execute_instr(lui);
        assert_eq!(cpu.get_register(1), 0x12345000);
    }

    #[test]
    fn test_auipc_instruction() {     
        let mut cpu = BasicCpu::new();
        cpu.init();
        // Test AUIPC (Add Upper Immediate to PC)
        let auipc = 0x12345097;   // auipc x1
        cpu.execute_instr(auipc);
        assert_eq!(cpu.get_register(1), (DRAM_BASE_ADDR + 0x12345000) as u64);
    }
    
    #[test]
    fn test_jal_instruction() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();

        // JAL x1, 1024 (jump forward 1024 bytes and store return address in x1)
        let jal = 0x400000EF;    // imm=1024, rd=x1
        cpu.execute_instr(jal);

        // Check return address stored in x1 (PC + 4)
        assert_eq!(cpu.get_register(1), (initial_pc + 4) as u64);
        // Check new PC value (PC + 1024)
        assert_eq!(cpu.get_pc(), initial_pc + 1024);
    }

    #[test]
    fn test_jalr_instruction() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();

        // Setup base address in x1
        cpu.set_register(1, (initial_pc + 100) as u64);
        
        // JALR x2, x1, 8 (jump to x1 + 8 and store return address in x2)
        let jalr = 0x00808167;    // imm=8, rs1=x1, rd=x2
        cpu.execute_instr(jalr);

        // Check return address stored in x2 (PC + 4)
        assert_eq!(cpu.get_register(2), (initial_pc + 4) as u64);
        // Check new PC value (x1 + 8)
        assert_eq!(cpu.get_pc(), initial_pc + 108);
    }

    #[test]
    fn test_branch_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();

        // Setup test values
        cpu.set_register(1, 5);  // x1 = 5
        cpu.set_register(2, 10); // x2 = 10
        cpu.set_register(3, 5);  // x3 = 5 (equal to x1)
        cpu.set_register(4, 0xFFFFFFFF); // x4 = -1 (signed)

        // Test BEQ (Branch if Equal)
        // BEQ x1, x3, 8 (branch if x1 == x3)
        let beq = 0x00308463;    // imm=8, rs1=x1, rs2=x3
        cpu.execute_instr(beq);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BNE (Branch if Not Equal)
        cpu.set_pc(initial_pc);
        // BNE x1, x2, 8 (branch if x1 != x2)
        let bne = 0x00209463;    // imm=8, rs1=x1, rs2=x2
        cpu.execute_instr(bne);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BLT (Branch if Less Than)
        cpu.set_pc(initial_pc);
        // BLT x1, x2, 8 (branch if x1 < x2)
        let blt = 0x0020C463;    // imm=8, rs1=x1, rs2=x2
        cpu.execute_instr(blt);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BGE (Branch if Greater or Equal)
        cpu.set_pc(initial_pc);
        // BGE x1, x3, 8 (branch if x1 >= x3)
        let bge = 0x0030D463;    // imm=8, rs1=x1, rs2=x3
        cpu.execute_instr(bge);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BLTU (Branch if Less Than Unsigned)
        cpu.set_pc(initial_pc);
        // BLTU x1, x4, 8 (branch if x1 < x4 unsigned)
        let bltu = 0x00406463;    // imm=8, rs1=x1, rs2=x4
        cpu.execute_instr(bltu);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch (5 < 0xFFFFFFFF unsigned)

        // Test BGEU (Branch if Greater or Equal Unsigned)
        cpu.set_pc(initial_pc);
        // BGEU x4, x1, 8 (branch if x4 >= x1 unsigned)
        let bgeu = 0x00127463;    // imm=8, rs1=x4, rs2=x1
        cpu.execute_instr(bgeu);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch (0xFFFFFFFF > 5 unsigned)

        // Test branch not taken
        cpu.set_pc(initial_pc);
        // BEQ x1, x2, 8 (branch if x1 == x2, which is false)
        let beq_not_taken = 0x00208463;
        cpu.execute_instr(beq_not_taken);
        assert_eq!(cpu.get_pc(), initial_pc + 4); // Should not branch, PC += 4
    }

    #[test]
    fn test_load_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Setup: Store test values in memory
        cpu.mem.dram_write(DRAM_BASE_ADDR, 32, 0xFF00FF00);
        cpu.mem.dram_write(DRAM_BASE_ADDR + 4, 64, 0xDEADBEEF12345678);
        // Set base address in register
        cpu.set_register(1, DRAM_BASE_ADDR as u64);

        // Test LB (Load Byte signed)
        let lb = 0x00008083;    // lb x1, 0(x1)
        cpu.execute_instr(lb);
        assert_eq!(cpu.get_register(1), 0x0); // Should sign extend 0x00

        // Test LH (Load Halfword signed)
        cpu.set_register(2, (DRAM_BASE_ADDR + 4) as u64);
        let lh = 0x00011103;    // lh x2, 0(x2)
        cpu.execute_instr(lh);
        assert_eq!(cpu.get_register(2), 0x5678); 

        // Test LW (Load Word signed)
        cpu.set_register(3, (DRAM_BASE_ADDR + 4) as u64);
        let lw = 0x0001A183;    // lw x3, 0(x3)
        cpu.execute_instr(lw);
        assert_eq!(cpu.get_register(3), 0x12345678); 

        // Test LBU (Load Byte unsigned)
        cpu.set_register(4, DRAM_BASE_ADDR as u64);
        let lbu = 0x00024203;   // lbu x4, 0(x4)
        cpu.execute_instr(lbu);
        assert_eq!(cpu.get_register(4), 0x00); // Should zero extend

        // Test LHU (Load Halfword unsigned)
        cpu.set_register(5, DRAM_BASE_ADDR as u64);
        let lhu = 0x0002D283;   // lhu x5, 0(x5)
        cpu.execute_instr(lhu);
        assert_eq!(cpu.get_register(5), 0xFF00); // Should zero extend
    }

    #[test]
    fn test_store_instructions() {
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Set base address in register
        cpu.set_register(1, DRAM_BASE_ADDR as u64);
        
        // Test SB (Store Byte)
        cpu.set_register(2, 0xFF);
        let sb = 0x00208023;    // sb x2, 0(x1)
        cpu.execute_instr(sb);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR, 8), 0xFF);

        // Test SH (Store Halfword)
        cpu.set_register(3, 0xABCD);
        let sh = 0x00309123;    // sh x3, 2(x1)
        cpu.execute_instr(sh);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 2, 16), 0xABCD);

        // Test SW (Store Word)
        cpu.set_register(4, 0x12345678);
        let sw = 0x0040A223;    // sw x4, 4(x1)
        cpu.execute_instr(sw);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 4, 32), 0x12345678);

        // Test overlapping stores
        cpu.set_register(5, 0xFF);
        let sb2 = 0x00508423;   // sb x5, 8(x1)
        cpu.execute_instr(sb2);
        
        cpu.set_register(6, 0xABCD);
        let sh2 = 0x00609423;   // sh x6, 8(x1)
        cpu.execute_instr(sh2);
        
        // Verify that the halfword write overwrote the previous byte write
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 8, 16), 0xABCD);
    }
}