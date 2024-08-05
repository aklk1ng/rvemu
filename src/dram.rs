use crate::exception::Exception;
use crate::param::{DRAM_BASE, DRAM_SIZE};

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    /// Create a new dram with code
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.into_iter());
        Self { dram }
    }

    /// Load data of size from memory
    // addr/size must be valid
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::LoadAccessFault(addr));
        }

        let nbytes = size / 8;
        let index = (addr - DRAM_BASE) as usize;
        let mut code = self.dram[index] as u64;
        // shift the bytes to build up the desired value
        for i in 1..nbytes {
            code |= (self.dram[index + i as usize] as u64) << (i * 8);
        }

        Ok(code)
    }

    /// Store value of size in memory
    // addr/size must be valid
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::LoadAccessFault(addr));
        }

        let nbytes = size / 8;
        let index = (addr - DRAM_BASE) as usize;
        for i in 0..nbytes {
            let offset = 8 * i as usize;
            self.dram[index + i as usize] = ((value >> offset) & 0xff) as u8;
        }
        Ok(())
    }

    /// Return the dram size
    pub fn dram_size(&self) -> usize {
        self.dram.len()
    }
}
