/*
 *  author:     earacena
 *  file:       opcode.rs
 *
 */

use crate::cpu::CPU;

#[derive(Debug)]
pub enum OpcodeError {
    NotImplemented,
}

pub type OpcodeResult = Result<Opcode, OpcodeError>;

pub struct Opcode {
    pub val: u8,
    pub func: fn(&mut CPU),
}

pub fn op_0x00(cpu: &mut CPU) {
        // 0x00 - NOP
        println!("Doing nothing @ PC {:#6x}", cpu.pc);
}
