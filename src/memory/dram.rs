pub const DRAM_SIZE: usize = 1024*1024*1;
pub const DRAM_BASE_ADDR: usize = 0x80000000;

pub struct DramMemory {
    pub mem : Vec<u8>
}

impl DramMemory {

    pub fn dram_read(&self, addr: usize, size: usize) -> u64{
        let mut value: u64 = 0;

        for shift in 0..size/8 {
            let b = u64::from(self.mem[addr-DRAM_BASE_ADDR+shift]) << shift*8;
            value |= b as u64;
        }

        value
    }

    pub fn dram_write(&mut self, addr: usize, size: usize, value: u64){

        for shift in 0..size/8 {
            self.mem[addr-DRAM_BASE_ADDR+shift] = ((value & 0xFF) << shift*8) as u8;
        }
    }
}