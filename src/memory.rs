/*
 * author:  earacena
 * file:    memory.rs
 *
 */

pub type Address = u16;
pub type Data = u8;
pub type MemoryReadResult = Result<Data, MemoryError>;
pub type MemoryWriteResult = Result<(), MemoryError>;

#[derive(Debug)]
pub enum MemoryError {
    UnusableMemory,
    UnaccessibleAddress,
}

pub struct Memory {
    // Memory Map
    pub rom: [Data; 0x8000],     // 0000 ~ 7FFF: ROM
    pub vram: [Data; 0x2000],    // 8000 ~ 9FFF: VRAM
    pub eram: [Data; 0x2000],    // A000 ~ BFFF: External RAM
    pub wram: [Data; 0x2000],    // C000 ~ DFFF: Work RAM
    pub echoram: [Data; 0x1E00], // E000 ~ FDFF: Mirror of C000~DDFF
    pub oam: [Data; 0x00A0],     // FE00 ~ FE9F: Sprite Attribute Table
    // FEA0 ~ FEFF: Not Usable
    pub io_reg: [Data; 0x0080], // FF00 ~ FF7F: I/O Registers
    pub hram: [Data; 0x007F],   // FF80 ~ FFFE: High RAM
    pub ie: Data,               // FFFF ~ FFFF: Interrupts Enable Register
}

impl Memory {
    pub fn initialize() -> Memory {
        let memory = Memory {
            rom: [0x00; 0x8000],
            vram: [0x00; 0x2000],
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            echoram: [0x00; 0x1E00],
            oam: [0x00; 0x00A0],
            io_reg: [0x00; 0x0080],
            hram: [0x00; 0x007F],
            ie: 0x00,
        };
        memory
    }

    pub fn read(&self, address: Address) -> MemoryReadResult {
        let index = usize::from(address);
        match index {
            0x0000..=0x7FFF => Ok(self.rom[index]),
            0x8000..=0x9FFF => Ok(self.vram[index - 0x8000]),
            0xA000..=0xBFFF => Ok(self.eram[index - 0xA000]),
            0xC000..=0xDFFF => Ok(self.wram[index - 0xC000]),
            0xE000..=0xFDFF => Ok(self.echoram[index - 0xE000]),
            0xFE00..=0xFE9F => Ok(self.oam[index - 0xFE00]),
            0xFEA0..=0xFEFF => Err(MemoryError::UnusableMemory),
            0xFF00..=0xFF7F => Ok(self.io_reg[index - 0xFF00]),
            0xFF80..=0xFFFE => Ok(self.hram[index - 0xFF80]),
            0xFFFF => Ok(self.ie),
            _ => Err(MemoryError::UnaccessibleAddress),
        }
    }

    pub fn write(&mut self, address: Address, data: Data) -> MemoryWriteResult {
        let index = usize::from(address);
        match index {
            0x0000..=0x7FFF => Ok(self.rom[index] = data),
            0x8000..=0x9FFF => Ok(self.vram[index - 0x8000] = data),
            0xA000..=0xBFFF => Ok(self.eram[index - 0xA000] = data),
            0xC000..=0xDFFF => Ok(self.wram[index - 0xC000] = data),
            0xE000..=0xFDFF => Ok(self.echoram[index - 0xE000] = data),
            0xFE00..=0xFE9F => Ok(self.oam[index - 0xFE00] = data),
            0xFEA0..=0xFEFF => Err(MemoryError::UnusableMemory),
            0xFF00..=0xFF7F => Ok(self.io_reg[index - 0xFF00] = data),
            0xFF80..=0xFFFE => Ok(self.hram[index - 0xFF80] = data),
            0xFFFF => Ok(self.ie = data),
            _ => Err(MemoryError::UnaccessibleAddress),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read() {
        let mut mem = Memory::initialize();

        mem.rom[0x65] = 0x23;
        match mem.read(0x65) {
            Ok(value) => assert_eq!(value, 0x23),
            Err(err) => panic!("{:?}, err"),
        };
    }

    #[test]
    fn test_write() {
        let mut mem = Memory::initialize();

        let address: Address = 0x35;
        let data: Data = 0xA1;
        mem.write(address, data);

        let index = usize::from(address);
        assert_eq!(mem.rom[index], data);
    }
}
