/*
 * author:  earacena
 * file:    memory.rs
 *
 */

pub struct Memory {
    pub ram: [u8; 65636],
}

impl Memory {
    pub fn initialize() -> [u8; 65636] {
        [0; 65636]
    }

    pub fn read(&self, address: u16) -> u8 {
        let index = usize::from(address);
        self.ram[index]
    }
    
    pub fn write(&mut self, address: u16, data: u8) {
        let index = usize::from(address);
        self.ram[index] = data;
    }
}
