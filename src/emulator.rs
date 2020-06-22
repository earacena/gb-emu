/*
 * author:  earacena
 * file:    emulator.rs
 */

use crate::cpu::CPU;
use crate::memory::{Data, Memory};
use crate::opcode;
use crate::opcode::{Opcode, OpcodeTable};

pub struct Emulator {
    pub cpu: CPU,
    pub memory: Memory,
}

impl Emulator {
    pub fn execute(&mut self) {
        let mut counter = 1000;
        println!("Executing...");
        let mut opcode: Opcode;
        let mut val: Data = 0x00;
        loop {
            println!("{:?}", self.cpu);
            val = self.cpu.fetch(&self.memory);
            opcode = self.cpu.decode(&self.memory, val);
            self.cpu.execute(opcode);

            if self.cpu.pc == 0x7FFF {
                break;
            }
        }
        println!("Exiting...");
    }
}
