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
}