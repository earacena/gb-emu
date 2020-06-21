/*
 * author:  earacena
 * file:    cpu.rs
 */

use crate::memory::Memory;
use crate::opcode;
use crate::opcode::Opcode;
use crate::opcode::OpcodeError;
use crate::opcode::OpcodeResult;
use std::fmt;

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
        };
        cpu
    }

    pub fn fetch(&mut self, mem: &Memory) -> u8 {
        let opcode = mem.read(self.pc);
        self.pc += 1;
        opcode
    }

    pub fn decode(&mut self, mem: &Memory, opcode_val: u8) -> OpcodeResult {
        let opcode: OpcodeResult = match opcode_val {
            0x00 => Ok(Opcode {
                val: opcode_val,
                func: opcode::op_0x00,
            }),
            _ => Err(OpcodeError::NotImplemented),
        };
        opcode
    }

    pub fn execute(&mut self, func: fn(&mut CPU)) {
        func(self)
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
        let mut opcode_val: u8 = cpu.fetch(&mem);
        assert_eq!(opcode_val, 0xAA);
        assert_eq!(cpu.pc, 0x0011);

        opcode_val = cpu.fetch(&mem);
        assert_eq!(opcode_val, 0xBB);
        assert_eq!(cpu.pc, 0x0012);
    }

    #[test]
    fn test_decode() {
        let mut cpu = CPU::initialize();
        let mut mem = Memory::initialize();

        mem.write(0x0010, 0x00);
        cpu.pc = 0x0010;
        let opcode_val: u8 = 0x00;
        let opcode: OpcodeResult = cpu.decode(&mem, opcode_val);

        // Check if the correct function was returned by casting it and
        // the function definition to pointers and checking if the pointers
        // are the same. Casting both to usize also works, and preferable
        // to using raw pointers.
        match opcode {
            Ok(opcode) => assert_eq!(opcode.func as usize, opcode::op_0x00 as usize),
            Err(error) => panic!("Error decoding opcode function: `{:?}`", error),
        }
    }
}
