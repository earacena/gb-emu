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

    let memory = Memory::initialize();
    let cpu = CPU::initialize();

    let emu = Emulator {
        memory,
        cpu,
    };

    emu.execute();
    println!("Exiting...");
}
