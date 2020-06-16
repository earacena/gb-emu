/*
 * author:  earacena
 * file:    emulator.rs
 */

//mod cpu;
//mod memory;


use crate::cpu::CPU;
use crate::memory::Memory;

pub struct Emulator {
    pub cpu: CPU, 
    pub memory: Memory,
}

impl Emulator {
    pub fn execute(&self) {
        println!("Executing...");
    }
}
