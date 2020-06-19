/*
 * author:  earacena
 * file:    cpu.rs
 */

use crate::opcode;
use crate::memory::Memory;


pub struct CPU {
    pub reg_a:  u8,
    pub reg_b:  u8,
    pub reg_c:  u8,
    pub reg_d:  u8,
    pub reg_e:  u8,
    pub reg_h:  u8,
    pub reg_l:  u8,
    pub flags:  u8,
    pub sp:     u16,
    pub pc:     u16,
}


impl CPU {
    pub fn initialize() -> CPU {
        let cpu = CPU {
            reg_a:  0x00,
            reg_b:  0x00,
            reg_c:  0x00,
            reg_d:  0x00,
            reg_e:  0x00,
            reg_h:  0x00,
            reg_l:  0x00,
            flags:  0x00,
            sp:     0x0000,
            pc:     0x0000,  
        };
        cpu
    }


    pub fn fetch(&mut self, mem: &Memory) -> u8 {
        let opcode = mem.read(self.pc);
        self.pc += 1;
        opcode
    }

    pub fn decode_and_execute(&mut self, mem: &Memory, opcode: u8) {
        match opcode {
            0x00 => opcode::op_0x00(self),
            _ => println!("Opcode [{:#6x}] not implemented.", opcode),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        let mut cpu = CPU::initialize();
        let mut mem = Memory::initialize();

        mem.write(0x0010, 0xAA);
        mem.write(0x0011, 0xBB);

        cpu.pc = 0x0010;
        let mut opcode: u8 = cpu.fetch(&mem);
        assert_eq!(opcode, 0xAA);
        assert_eq!(cpu.pc, 0x0011);

        opcode = cpu.fetch(&mem);
        assert_eq!(opcode, 0xBB);
        assert_eq!(cpu.pc, 0x0012);
    }


    fn test_decode_and_execute() {

    }
}
