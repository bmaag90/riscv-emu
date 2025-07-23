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
        assert_eq!(cpu.get_pc(), DRAM_BASE_ADDR);
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
        cpu.set_register(3, 0xFFFFFFFF);  // -1 in two's complement
        let srai = 0x41f1d193;    // srai x3, x3, 31
        cpu.execute_instr(srai);
        assert_eq!(cpu.get_register(3), 0xFFFFFFFF);  // Should remain -1
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
        assert_eq!(cpu.get_register(1), (DRAM_BASE_ADDR + 0x12345000) as u32);
    }
    
}