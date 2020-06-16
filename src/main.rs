/*
 * author:  earacena
 * file:    main.rs
 */

//mod cpu;
mod cpu;
mod emulator;
mod memory;


use crate::cpu::CPU;
use crate::emulator::Emulator;
use crate::memory::Memory;


fn main() {

    let memory = Memory {
        ram: [0x00; 0x10000],
    };

    let cpu = CPU {
        reg_a: 0x00,
        reg_b: 0x00,
        reg_c: 0x00,
        reg_d: 0x00,
        reg_e: 0x00,
        reg_h: 0x00,
        reg_l: 0x00,
        flags: 0x00,
        sp:    0x0000,
        pc:    0x0000,
    };

    let emu = Emulator {
        memory,
        cpu,
    };

    emu.execute();
    println!("Exiting...");
}
