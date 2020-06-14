/*
 * author:  earacena
 * file:    main.rs
 */

//mod cpu;
mod memory;

use crate::memory::Memory;

struct Emulator {
    cpu: CPU, 
    memory: Memory,
}

impl Emulator {
    fn execute(&self) {
        println!("Executing...");
    }
}

fn main() {
    let emu = Emulator {
        memory: Memory { ram: Memory::initialize() },
        cpu: CPU {

        },
    };
    emu.execute();
    println!("Exiting...");
}
