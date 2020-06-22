/*
 *  author:     earacena
 *  file:       opcode.rs
 *
 */

use crate::cpu::CPU;

pub type Opcode = fn(&mut CPU);

pub struct OpcodeTable {
    pub op: [Opcode; 0x100],
    pub cb: [Opcode; 0x100],
}

impl OpcodeTable {
    pub fn load_tables() -> OpcodeTable {
        let op: [Opcode; 0x100] = [op_0x00; 0x100];
        let cb: [Opcode; 0x100] = [op_0x00; 0x100];

        OpcodeTable { op, cb }
    }
}

pub fn op_0x00(cpu: &mut CPU) {
    // 0x00 - NOP
    println!("--> [0x00]: Doing nothing @ PC: {:#06x}\n", cpu.pc - 1);
}
