/*
 * author:  earacena
 * file:    cpu.rs
 */

use crate::memory::{Memory, MemoryReadResult, MemoryWriteResult};
use crate::opcode;
use crate::opcode::{Opcode, OpcodeTable};
use std::fmt;

type OpcodeValue = u8;
type Data = u8;

pub struct CPU {
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_h: u8,
    pub reg_l: u8,
    pub flags: u8,
    pub sp: u16,
    pub pc: u16,

    pub table: OpcodeTable,
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            concat!(
                "CPU [A: {:#04x} B:{:#04x} C: {:#04x} ",
                "D: {:#04x} E: {:#04x} H:{:#04x} L: {:#04x}",
                " Flags: {:#04x} SP: {:#06x} PC: {:#06x}]"
            ),
            self.reg_a,
            self.reg_b,
            self.reg_c,
            self.reg_d,
            self.reg_e,
            self.reg_h,
            self.reg_l,
            self.flags,
            self.sp,
            self.pc
        )
    }
}

impl CPU {
    pub fn initialize() -> CPU {
        let cpu = CPU {
            reg_a: 0x00,
            reg_b: 0x00,
            reg_c: 0x00,
            reg_d: 0x00,
            reg_e: 0x00,
            reg_h: 0x00,
            reg_l: 0x00,
            flags: 0x00,
            sp: 0x0000,
            pc: 0x0000,

            table: OpcodeTable::load_tables(),
        };
        cpu
    }

    pub fn fetch(&mut self, mem: &Memory) -> Data {
        let result = match mem.read(self.pc) {
            Ok(value) => value,
            Err(err) => panic!("{:?}", err),
        };
        self.pc += 1;
        result
    }

    pub fn decode(&mut self, mem: &Memory, value: OpcodeValue) -> Opcode {
        let index = usize::from(value);
        match index {
            0x00..=0xCA | 0xCC..=0xFF => self.table.op[index],
            0xCB => self.table.cb[usize::from(self.fetch(&mem))],
            _ => panic!("Invalid memory address during decoding: {:#06}", index),
        }
    }

    pub fn execute(&mut self, opcode: Opcode) {
        opcode(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        let mut cpu = CPU::initialize();
        let mut mem = Memory::initialize();

        match mem.write(0x0010, 0xAA) {
            Ok(()) => (),
            Err(err) => panic!("{:?}", err),
        };

        match mem.write(0x0011, 0xBB) {
            Ok(()) => (),
            Err(err) => panic!("{:?}", err),
        };

        cpu.pc = 0x0010;
        let mut value: OpcodeValue = cpu.fetch(&mem);
        assert_eq!(value, 0xAA);
        assert_eq!(cpu.pc, 0x0011);

        value = cpu.fetch(&mem);
        assert_eq!(value, 0xBB);
        assert_eq!(cpu.pc, 0x0012);
    }

    #[test]
    fn test_decode() {
        let mut cpu = CPU::initialize();
        let mut mem = Memory::initialize();

        let result = match mem.write(0x0010, 0x00) {
            Ok(()) => (),
            Err(err) => panic!("{:?}", err),
        };

        cpu.pc = 0x0010;
        let value: OpcodeValue = 0x00;
        let opcode: Opcode = cpu.decode(&mem, value);

        // Check if the correct function was returned by casting it and
        // the function definition to pointers and checking if the pointers
        // are the same. Casting both to usize also works, and preferable
        // to using raw pointers.
        assert_eq!(opcode as usize, opcode::op_0x00 as usize);
    }
}
