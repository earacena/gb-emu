/*
 * author:  earacena
 * file:    cpu.rs
 */

struct CPU {
    reg_A:  u8,
    reg_B:  u8,
    reg_C:  u8,
    reg_D:  u8,
    reg_E:  u8,
    reg_H:  u8,
    reg_L:  u8,
    flags:  u8,
    sp:     u16,
    pc:     u16,
}


impl CPU {
    fn fetch(&mut self, instructions: Vec<u8>) -> u8 {
        let opcode = instructions[self.pc];
        self.pc += 1;
        opcode
    }

    //  fn decode(&self, opcode: u8) -> fn(cpu: CPU) {
    //
    //  }
    //
    //  fn execute(mut &self, fn(cpu: CPU)) {
    //
    //  }
}
