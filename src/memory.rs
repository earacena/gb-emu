/*
 * author:  earacena
 * file:    memory.rs
 *
 */

pub struct Memory {
    pub ram: [u8; 0xFFFF],
}

impl Memory {

    pub fn read(&self, address: u16) -> u8 {
        let index = usize::from(address);
        self.ram[index]
    }
    
    pub fn write(&mut self, address: u16, data: u8) {
        let index = usize::from(address);
        self.ram[index] = data;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read() {
        let mut mem = Memory { ram: [0x00; 0x10000], };

        mem.ram[0x65] = 0x23;
        assert_eq!(mem.read(0x65), 0x23);
    }

    #[test]
    fn test_write() {
        let mut mem = Memory {ram: [0x00; 0x10000], };

        let address: u16 = 0x35;
        let data: u8 = 0xA1; 
        mem.write(address, data);

        let index = usize::from(address);
        assert_eq!(mem.ram[index], data);
    }

}
