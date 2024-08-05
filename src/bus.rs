use crate::dram::Dram;
use crate::exception::Exception;
use crate::param::{DRAM_BASE, DRAM_END};

pub struct Bus {
    dram: Dram,
}

impl Bus {
    /// Create a new Bus with code
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(code),
        }
    }

    /// Check the address and load on dram
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }

    /// Check the address and store on dram
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }
}
