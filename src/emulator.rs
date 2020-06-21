/*
 * author:  earacena
 * file:    emulator.rs
 */

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::opcode;
use crate::opcode::Opcode;
use crate::opcode::OpcodeResult;

pub struct Emulator {
    pub cpu: CPU,
    pub memory: Memory,
}

impl Emulator {
    pub fn execute(&mut self) {
        let mut counter = 1000;
        println!("Executing...");

        let mut opcode_val: u8 = 0x00;
        let mut result: OpcodeResult;
        let mut opcode: Opcode;
        loop {
            println!("{:?}", self.cpu);
            opcode_val = self.cpu.fetch(&self.memory);
            result = self.cpu.decode(&self.memory, opcode_val);
            opcode = match result {
                Ok(opcode) => opcode,
                Err(err) => panic!("Error decoding opcode: {:?}", err),
            };

            self.cpu.execute(opcode.func);

            if self.cpu.pc == 0x10 {
                break;
            }
        }
        println!("Exiting...");
    }
}
