/*
 * author:  earacena
 * file:    main.rs
 */

mod cpu;
mod emulator;
mod memory;
mod opcode;

use crate::cpu::CPU;
use crate::emulator::Emulator;
use crate::memory::Memory;

fn main() {
    let mut memory = Memory::initialize();
    let mut cpu = CPU::initialize();

    let mut emu = Emulator { memory, cpu };

    emu.execute();
}
