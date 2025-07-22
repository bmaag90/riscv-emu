use riscv_emu::memory::dram::{DramMemory, DRAM_SIZE, DRAM_BASE_ADDR};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dram_write_read_byte() {
        let mut dram = DramMemory {
            mem: vec![0; DRAM_SIZE]
        };

        // Write and read a single byte
        dram.dram_write(DRAM_BASE_ADDR, 8, 0xAB);
        let value = dram.dram_read(DRAM_BASE_ADDR, 8);
        assert_eq!(value, 0xAB);
    }

    #[test]
    fn test_dram_write_read_word() {
        let mut dram = DramMemory {
            mem: vec![0; DRAM_SIZE]
        };

        // Write and read a 32-bit word
        dram.dram_write(DRAM_BASE_ADDR, 32, 0x12345678);
        let value = dram.dram_read(DRAM_BASE_ADDR, 32);
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn test_dram_multiple_addresses() {
        let mut dram = DramMemory {
            mem: vec![0; DRAM_SIZE]
        };

        // Write to multiple addresses
        dram.dram_write(DRAM_BASE_ADDR, 8, 0xAB);
        dram.dram_write(DRAM_BASE_ADDR + 1, 8, 0xCD);
        
        // Read from multiple addresses
        let value1 = dram.dram_read(DRAM_BASE_ADDR, 8);
        let value2 = dram.dram_read(DRAM_BASE_ADDR + 1, 8);
        
        assert_eq!(value1, 0xAB);
        assert_eq!(value2, 0xCD);
    }

    #[test]
    fn test_dram_byte_alignment() {
        let mut dram = DramMemory {
            mem: vec![0; DRAM_SIZE]
        };

        // Write 32-bit value
        dram.dram_write(DRAM_BASE_ADDR, 32, 0xAABBCCDD);
        
        // Read individual bytes
        let byte0 = dram.dram_read(DRAM_BASE_ADDR, 8);
        let byte1 = dram.dram_read(DRAM_BASE_ADDR + 1, 8);
        let byte2 = dram.dram_read(DRAM_BASE_ADDR + 2, 8);
        let byte3 = dram.dram_read(DRAM_BASE_ADDR + 3, 8);

        assert_eq!(byte0, 0xDD);
        assert_eq!(byte1, 0xCC);
        assert_eq!(byte2, 0xBB);
        assert_eq!(byte3, 0xAA);
    }

    #[test]
    #[should_panic]
    fn test_invalid_address() {
        let dram = DramMemory {
            mem: vec![0; DRAM_SIZE]
        };

        // Try to read from invalid address
        dram.dram_read(0x0, 8);
    }
}