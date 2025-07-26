use riscv_emu::cpu::basic_cpu::BasicCpu;
use riscv_emu::memory::dram::DRAM_BASE_ADDR;
use log::info;
use env_logger;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_cpu_initialization() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Check if register x0 is zero (hardwired)
        assert_eq!(cpu.get_register(0), 0);
        
        // Check if PC is initialized to DRAM base address
        assert_eq!(cpu.get_pc(), DRAM_BASE_ADDR.try_into().unwrap());
    }

    #[test]
    fn test_register_operations() {
        test_init();
        let mut cpu = BasicCpu::new();
        
        cpu.init();

        for i in 1..32 {
            cpu.set_register(i, i as u64);
            assert_eq!(cpu.get_register(i), i as u64);
        }
        
    }

    #[test]
    fn test_pc_operations() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();
        
        assert_eq!(initial_pc, DRAM_BASE_ADDR.try_into().unwrap());

        // Test PC increment
        cpu.set_pc(initial_pc + 4);
        assert_eq!(cpu.get_pc(), initial_pc + 4);
    }
    
    #[test]
    fn test_instruction_fetch() {
        test_init();
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
        test_init();
        let cpu = BasicCpu::new();
        
        // Test instruction: addi x2, x1, 5
        // Format: imm[11:0] rs1 000 rd 0010011
        let instruction = 0x00508113;
        
        assert_eq!(cpu.instr_opcode(instruction), 0b0010011);
        assert_eq!(cpu.instr_rd(instruction), 2);       // x2
        assert_eq!(cpu.instr_func3(instruction), 0);    // addi
        assert_eq!(cpu.instr_rs1(instruction), 1);      // x1
        assert_eq!(cpu.instr_imm_i(instruction), 5);    // immediate value

        let instruction = 0xFF508113;
        assert_eq!(cpu.instr_imm_i(instruction) as i64, -11); // immediate value in two's complement
        let instruction = 0xFFFFF097;
        assert_eq!(cpu.instr_imm_u(instruction) as i64, -4096);
        let instruction = 0xFF40A223;
        assert_eq!(cpu.instr_imm_s(instruction) as i64, -28);
        let instruction = 0xFE308F63;
        assert_eq!(cpu.instr_imm_b(instruction) as i64, -2050);
        let instruction = 0xFFFFE0EF;
        assert_eq!(cpu.instr_imm_j(instruction) as i64, -4098);
    }

    #[test]
    fn test_addi_execution() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Set initial value in source register
        cpu.set_register(1, 5);
        
        // Create ADDI instruction: add 3 to x1 and store in x2
        let instruction = 0x00308113; // addi x2, x1, 3
        
        let _ = cpu.execute_instr(instruction);
        
        // Check result: 5 + 3 = 8
        assert_eq!(cpu.get_register(2), 8);

        let instruction = 0xFF308113; // addi x2, x1, -13
        let _ = cpu.execute_instr(instruction);
        // Check result: 5 - 13 = -8 (0xFFFFFFF8 in two's complement)
        assert_eq!(cpu.get_register(2) as i64, -8);
    }

    #[test]
    fn test_shift_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test SLLI (Shift Left Logical Immediate)
        cpu.set_register(1, 0x1);
        let slli = 0x00109093;    // slli x1, x1, 1
        let _ = cpu.execute_instr(slli);
        assert_eq!(cpu.get_register(1), 0x2);

        // Test SRLI (Shift Right Logical Immediate)
        cpu.set_register(2, 0x8);
        let srli = 0x00115113;    // srli x2, x2, 1
        let _ = cpu.execute_instr(srli);
        assert_eq!(cpu.get_register(2), 0x4);

        // Test SRAI (Shift Right Arithmetic Immediate)
        cpu.set_register(3, 0xFFFFFFFFFFFFFFFF);  // -1 in two's complement
        let srai = 0x41f1d193;    // srai x3, x3, 31
        let _ = cpu.execute_instr(srai);
        assert_eq!(cpu.get_register(3), 0xFFFFFFFFFFFFFFFF);  // Should remain -1
    }

    #[test]
    fn test_set_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test SLTI (Set Less Than Immediate)
        cpu.set_register(1, 5);
        let slti = 0x00a0a093;    // slti x1, x1, 10
        let _ = cpu.execute_instr(slti);
        assert_eq!(cpu.get_register(1), 1);  // 5 < 10 -> true (1)

        // Test SLTIU (Set Less Than Immediate Unsigned)
        cpu.set_register(2, 10);
        let sltiu = 0x00513113;    // sltiu x2, x2, 5
        let _ = cpu.execute_instr(sltiu);
        assert_eq!(cpu.get_register(2), 0);  // 10 < 5 -> false (0)
    }

    #[test]
    fn test_logical_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test XORI
        cpu.set_register(1, 0xFF);
        let xori = 0x00f0c093;    // xori x1, x1, 15
        let _ = cpu.execute_instr(xori);
        assert_eq!(cpu.get_register(1), 0xF0);

        // Test ORI
        cpu.set_register(2, 0xF0);
        let ori = 0x00f16113;     // ori x2, x2, 15
        let _ = cpu.execute_instr(ori);
        assert_eq!(cpu.get_register(2), 0xFF);

        // Test ANDI
        cpu.set_register(3, 0xFF);
        let andi = 0x00f1f193;    // andi x3, x3, 15
        let _ = cpu.execute_instr(andi);
        assert_eq!(cpu.get_register(3), 0x0F);
    }

    #[test]
    fn test_lui_instruction() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test LUI (Load Upper Immediate)
        let lui = 0x123450b7;     // lui x1, 0x12345
        let _ = cpu.execute_instr(lui);
        assert_eq!(cpu.get_register(1), 0x12345000);
    }

    #[test]
    fn test_auipc_instruction() {     
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        // Test AUIPC (Add Upper Immediate to PC)
        let auipc = 0x12345097;   // auipc x1
        let _ = cpu.execute_instr(auipc);
        assert_eq!(cpu.get_register(1), (DRAM_BASE_ADDR + 0x12345000) as u64);
    }
    
    #[test]
    fn test_jal_instruction() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();

        // JAL x1, 1024 (jump forward 1024 bytes and store return address in x1)
        let jal = 0x400000EF;    // imm=1024, rd=x1
        let _ = cpu.execute_instr(jal);

        // Check return address stored in x1 (PC + 4)
        assert_eq!(cpu.get_register(1), (initial_pc + 4) as u64);
        // Check new PC value (PC + 1024)
        assert_eq!(cpu.get_pc(), initial_pc + 1024);
    }

    #[test]
    fn test_jalr_instruction() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        let initial_pc = cpu.get_pc();

        // Setup base address in x1
        cpu.set_register(1, (initial_pc + 100) as u64);
        
        // JALR x2, x1, 8 (jump to x1 + 8 and store return address in x2)
        let jalr = 0x00808167;    // imm=8, rs1=x1, rd=x2
        let _ = cpu.execute_instr(jalr);

        // Check return address stored in x2 (PC + 4)
        assert_eq!(cpu.get_register(2), (initial_pc + 4) as u64);
        // Check new PC value (x1 + 8)
        assert_eq!(cpu.get_pc(), initial_pc + 108);
    }

    #[test]
    fn test_branch_instructions() {
        test_init();
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
        let _ = cpu.execute_instr(beq);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BNE (Branch if Not Equal)
        cpu.set_pc(initial_pc);
        // BNE x1, x2, 8 (branch if x1 != x2)
        let bne = 0x00209463;    // imm=8, rs1=x1, rs2=x2
        let _ = cpu.execute_instr(bne);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BLT (Branch if Less Than)
        cpu.set_pc(initial_pc);
        // BLT x1, x2, 8 (branch if x1 < x2)
        let blt = 0x0020C463;    // imm=8, rs1=x1, rs2=x2
        let _ = cpu.execute_instr(blt);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BGE (Branch if Greater or Equal)
        cpu.set_pc(initial_pc);
        // BGE x1, x3, 8 (branch if x1 >= x3)
        let bge = 0x0030D463;    // imm=8, rs1=x1, rs2=x3
        let _ = cpu.execute_instr(bge);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch

        // Test BLTU (Branch if Less Than Unsigned)
        cpu.set_pc(initial_pc);
        // BLTU x1, x4, 8 (branch if x1 < x4 unsigned)
        let bltu = 0x00406463;    // imm=8, rs1=x1, rs2=x4
        let _ = cpu.execute_instr(bltu);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch (5 < 0xFFFFFFFF unsigned)

        // Test BGEU (Branch if Greater or Equal Unsigned)
        cpu.set_pc(initial_pc);
        // BGEU x4, x1, 8 (branch if x4 >= x1 unsigned)
        let bgeu = 0x00127463;    // imm=8, rs1=x4, rs2=x1
        let _ = cpu.execute_instr(bgeu);
        assert_eq!(cpu.get_pc(), initial_pc + 8); // Should branch (0xFFFFFFFF > 5 unsigned)

        // Test branch not taken
        cpu.set_pc(initial_pc);
        // BEQ x1, x2, 8 (branch if x1 == x2, which is false)
        let beq_not_taken = 0x00208463;
        let _ = cpu.execute_instr(beq_not_taken);
        assert_eq!(cpu.get_pc(), initial_pc + 4); // Should not branch, PC += 4
    }

    #[test]
    fn test_load_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        
        // Setup: Store test values in memory
        cpu.mem.dram_write(DRAM_BASE_ADDR, 32, 0xFF00FF00);
        cpu.mem.dram_write(DRAM_BASE_ADDR + 4, 64, 0xDEADBEEF12345678);
        // Set base address in register
        cpu.set_register(1, DRAM_BASE_ADDR as u64);

        // Test LB (Load Byte signed)
        let lb = 0x00008083;    // lb x1, 0(x1)
        let _ = cpu.execute_instr(lb);
        assert_eq!(cpu.get_register(1), 0x0); // Should sign extend 0x00

        // Test LH (Load Halfword signed)
        cpu.set_register(2, (DRAM_BASE_ADDR + 4) as u64);
        let lh = 0x00011103;    // lh x2, 0(x2)
        let _ = cpu.execute_instr(lh);
        assert_eq!(cpu.get_register(2), 0x5678); 

        // Test LW (Load Word signed)
        cpu.set_register(3, (DRAM_BASE_ADDR + 4) as u64);
        let lw = 0x0001A183;    // lw x3, 0(x3)
        let _ = cpu.execute_instr(lw);
        assert_eq!(cpu.get_register(3), 0x12345678); 

        // Test LBU (Load Byte unsigned)
        cpu.set_register(4, DRAM_BASE_ADDR as u64);
        let lbu = 0x00024203;   // lbu x4, 0(x4)
        let _ = cpu.execute_instr(lbu);
        assert_eq!(cpu.get_register(4), 0x00); // Should zero extend

        // Test LHU (Load Halfword unsigned)
        cpu.set_register(5, DRAM_BASE_ADDR as u64);
        let lhu = 0x0002D283;   // lhu x5, 0(x5)
        let _ = cpu.execute_instr(lhu);
        assert_eq!(cpu.get_register(5), 0xFF00); // Should zero extend
    }

    #[test]
    fn test_store_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Set base address in register
        cpu.set_register(1, DRAM_BASE_ADDR as u64);
        
        // Test SB (Store Byte)
        cpu.set_register(2, 0xFF);
        let sb = 0x00208023;    // sb x2, 0(x1)
        let _ = cpu.execute_instr(sb);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR, 8), 0xFF);

        // Test SH (Store Halfword)
        cpu.set_register(3, 0xABCD);
        let sh = 0x00309123;    // sh x3, 2(x1)
        let _ = cpu.execute_instr(sh);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 2, 16), 0xABCD);

        // Test SW (Store Word)
        cpu.set_register(4, 0x12345678);
        let sw = 0x0040A223;    // sw x4, 4(x1)
        let _ = cpu.execute_instr(sw);
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 4, 32), 0x12345678);

        // Test overlapping stores
        cpu.set_register(5, 0xFF);
        let sb2 = 0x00508423;   // sb x5, 8(x1)
        let _ = cpu.execute_instr(sb2);
        
        cpu.set_register(6, 0xABCD);
        let sh2 = 0x00609423;   // sh x6, 8(x1)
        let _ = cpu.execute_instr(sh2);
        
        // Verify that the halfword write overwrote the previous byte write
        assert_eq!(cpu.mem.dram_read(DRAM_BASE_ADDR + 8, 16), 0xABCD);
    }

    #[test]
    fn test_r_type_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        // Set initial values in registers
        cpu.set_register(1, 10); // x1 = 10
        cpu.set_register(2, 5);  // x2 = 5
       
        cpu.print_registers();
        // Test ADD
        let add = 0x002080B3;    // add x1, x1, x2
        let _ = cpu.execute_instr(add);
        assert_eq!(cpu.get_register(1), 15); 
        // Test SUB
        cpu.set_register(1, 10); // Reset x1 to 15
        let sub = 0x402080B3;    // sub x1, x1, x2
        let _ = cpu.execute_instr(sub);
        assert_eq!(cpu.get_register(1), 5); // 10 - 5

        // Test SLL
        cpu.set_register(1, 10); // Reset x1 to 10
        let sll = 0x002090B3;    // sll x1, x1, x2
        let _ = cpu.execute_instr(sll);
        assert_eq!(cpu.get_register(1), 320); // 10 << 5 

        // Test SLT
        cpu.set_register(1, 5);  // Reset x1 to 5
        let slt = 0x0020A0B3;    // slt x1, x1, x2
        let _ = cpu.execute_instr(slt);
        assert_eq!(cpu.get_register(1), 0); // 5 < 5
        cpu.set_register(1, 4);  // Reset x1 to 4
        let _ = cpu.execute_instr(slt);
        assert_eq!(cpu.get_register(1), 1); // 4 < 5

        // Test SLTU
        cpu.set_register(1, 5);  // Reset x1 to 5
        let sltu = 0x0020B0B3;   // sl
        let _ = cpu.execute_instr(sltu);
        assert_eq!(cpu.get_register(1), 0); // 5 < 5
        cpu.set_register(1, 4);  // Reset x1 to 4
        let _ = cpu.execute_instr(sltu);
        assert_eq!(cpu.get_register(1), 1); // 4 < 5

        // Test XOR
        cpu.set_register(1, 0xF0F0); // Reset x1
        cpu.set_register(2, 0x0F00); // Reset x2
        let xor = 0x0020C0B3;    // xor x1, x1, x2
        let _ = cpu.execute_instr(xor);
        assert_eq!(cpu.get_register(1), 0xFFF0); // 0xF0F0 ^ 0x0F00 = 0xFFF0
    }

    #[test]
    fn test_fence_instruction() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init(); 
        // Test FENCE instruction
        let fence = 0x0000000F; // FENCE instruction (no specific
        let _ = cpu.execute_instr(fence);
        // No specific state change expected, just checking execution
        assert!(true); // If we reach here, the test passes
    }

    #[test]
    fn test_system_instruction() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();
        // Test SYSTEM instruction (e.g. ECALL, EBREAK)
        let system = 0x00000073; // SYSTEM instruction (no specific operation)
        let _ = cpu.execute_instr(system);
        // No specific state change expected, just checking execution
        assert!(true); // If we reach here, the test passes
    }

    #[test]
    fn test_csr_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test CSRRW (CSR Read and Write)
        let csr_addr = 0x305;  // mtvec CSR address
        cpu.set_register(18, 0xAAAA_AAAA);
        let csrrw = 0x30591573;  // csrrw x10 (rd), x18 (rs)
        let _ = cpu.execute_instr(csrrw);
        assert_eq!(cpu.get_csr(csr_addr), 0xAAAA_AAAA);
        assert_eq!(cpu.get_register(10), 0); // Should read old value (0) into x10

        // Test CSRRS (CSR Read and Set)
        cpu.set_register(18, 0x5555_5555);
        let csrrs = 0x30592573;  // csrrs x10 (rd), x18 (rs)
        let _ = cpu.execute_instr(csrrs);
        assert_eq!(cpu.get_csr(csr_addr), 0xAAAA_AAAA | 0x5555_5555);
        assert_eq!(cpu.get_register(10), 0xAAAA_AAAA); // Should read old value into x10

        // Test CSRRC (CSR Read and Clear)
        cpu.set_register(18, 0xF0F0_F0F0);
        let csrrc = 0x30593573;  // csrrc x10 (rd), x18 (rs)
        let _ = cpu.execute_instr(csrrc);
        // Should read old value into x3 and clear bits
        assert_eq!(cpu.get_csr(csr_addr), 0x0F0F_0F0F); // 0xAAAA_AAAA | 0x5555_5555 = 0xFFFF_FFFF --> 0xFFFF_FFFF & ~0xF0F0_F0F0
        assert_eq!(cpu.get_register(10), 0xAAAA_AAAA | 0x5555_5555); // Should read old value into x10

        // Test CSRRWI (CSR Read and Write Immediate)
        let csrrwi = 0x30595573;  // csrrwi x10 (rd), 18 (rs = uimm)
        let _ = cpu.execute_instr(csrrwi);
        assert_eq!(cpu.get_csr(csr_addr), 18); // Should write immediate value 18 to CSR
        assert_eq!(cpu.get_register(10), 0x0F0F_0F0F); // Should read old value into x10

        // Test CSRRSI (CSR Read and Set Immediate)
        let csrrsi = 0x305A6573;  // csrrsi x10 (rd), 20 (rs = uimm)
        let _ = cpu.execute_instr(csrrsi);
        // Should read old value into x2 and set immediate bits
        assert_eq!(cpu.get_csr(csr_addr), 22); // 18 | 20 = 22
        assert_eq!(cpu.get_register(10), 18); 

        // Test CSRRCI (CSR Read and Clear Immediate)
        let csrrci = 0x305C7573;  // csrrci x10 (rd), 24 (rs = uimm)
        let _ = cpu.execute_instr(csrrci);
        // Should read old value into x3 and clear immediate bits
        assert_eq!(cpu.get_register(10), 22);
        assert_eq!(cpu.get_csr(csr_addr), 22 & !24);  // 22 & ~24 

        // Test CSR read-only behavior with x0
        cpu.set_csr(csr_addr, 0xDEAD_BEEF);
        let csrrw_x0 = 0x30501573;  // csrrw x10, x0
        let _ = cpu.execute_instr(csrrw_x0);
        // Should not modify CSR when rs1 = x0
        assert_eq!(cpu.get_csr(csr_addr), 0xDEAD_BEEF);
    }

        #[test]
    fn test_rv64i_extension_instructions() {
        test_init();
        let mut cpu = BasicCpu::new();
        cpu.init();

        // Test ADDW
        cpu.set_register(1, 0x00000000FFFFFFFF);  // Set x1 to max 32-bit value
        cpu.set_register(2, 0x0000000000000001);  // Set x2 to 1
        let addw = 0x002080BB;    // addw x1, x1, x2
        let _ = cpu.execute_instr(addw);
        // Result should be sign-extended to 64 bits
        assert_eq!(cpu.get_register(1), 0x100000000);  

        // Test SUBW
        cpu.set_register(3, 10);  // Set x3 to 10
        cpu.set_register(4, 11);  // Set x4 to 1
        let subw = 0x4041813B;    // subw x2, x3, x4
        let _ = cpu.execute_instr(subw);
        assert_eq!(cpu.get_register(2) as i64, -1);  // 10 - 11 = -1 

        // Test SLLW (Shift Left Logical Word)
        cpu.set_register(5, 0x0000000000000001);  // Set x5 to 1
        cpu.set_register(6, 0x0000000000000004);  // Set x6 to 4 (shift amount)
        let sllw = 0x0062923B;    // sllw x4, x5, x6
        let _ = cpu.execute_instr(sllw);
        assert_eq!(cpu.get_register(4), 16);  // 1 << 4 = 16

        // Test SRLW (Shift Right Logical Word)
        cpu.set_register(7, 0x0000_0000_1000_0000);  // Set x7 to large value
        cpu.set_register(8, 0x0000000000000004);  // Set x8 to 4 (shift amount)
        let srlw = 0x0083D33B;    // srlw x6, x7, x8
        let _ = cpu.execute_instr(srlw);
        assert_eq!(cpu.get_register(6), 0x100_0000);  // Logical shift, zeros inserted

        // Test SRAW (Shift Right Arithmetic Word)
        cpu.set_register(9, 0xFFFFFFFF80000000);  // Set x9 to negative value
        cpu.set_register(10, 0x0000000000000004); // Set x10 to 4 (shift amount)
        let sraw = 0x40A4D43B;    // sraw x8, x9, x10
        let _ = cpu.execute_instr(sraw);
        // Arithmetic shift should preserve sign bit
        assert_eq!(cpu.get_register(8), 0xFFFFFFFFF8000000);


        // Test SUBW underflow
        cpu.set_register(13, 0x0000000000000000);
        cpu.set_register(14, 0x0000000000000001);
        let subw_underflow = 0x40E686BB;    // subw x13, x13, x14
        let _ = cpu.execute_instr(subw_underflow);
        // Result should be -1 sign-extended to 64 bits
        assert_eq!(cpu.get_register(13) as i64, -1);

        // Test SLLW with max shift
        cpu.set_register(15, 0x0000000000000001);
        cpu.set_register(16, 0x000000000000001F);  // Shift by 31 (max valid shift)
        let sllw_max = 0x010797BB;    // sllw x15, x15, x16
        let _ = cpu.execute_instr(sllw_max);
        assert_eq!(cpu.get_register(15), 0x0000000080000000);
    }
}