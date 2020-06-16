/*
 * author:  earacena
 * file:    cpu.rs
 */

pub struct CPU {
    pub reg_a:  u8,
    pub reg_b:  u8,
    pub reg_c:  u8,
    pub reg_d:  u8,
    pub reg_e:  u8,
    pub reg_h:  u8,
    pub reg_l:  u8,
    pub flags:  u8,
    pub sp:     u16,
    pub pc:     u16,
}


impl CPU {

    pub fn fetch(&mut self, instructions: &[u8]) -> u8 {
        let index = usize::from(self.pc);
        let opcode = instructions[index];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {

        let mut cpu = CPU {
            reg_a:  0x00,
            reg_b:  0x00,
            reg_c:  0x00,
            reg_d:  0x00,
            reg_e:  0x00,
            reg_h:  0x00,
            reg_l:  0x00,
            flags:  0x00,
            sp:     0x0000,
            pc:     0x0000,
        };

        let instructions = vec![0xAA, 0xBB, 0xCC, 0xDD];

        assert_eq!(cpu.pc, 0x0000);

        let mut opcode = cpu.fetch(instructions.as_slice());
        assert_eq!(opcode, 0xAA);
        assert_eq!(cpu.pc, 0x0001);

        opcode = cpu.fetch(instructions.as_slice());
        assert_eq!(opcode, 0xBB);
        assert_eq!(cpu.pc, 0x0002);

        opcode = cpu.fetch(instructions.as_slice());
        assert_eq!(opcode, 0xCC);
        assert_eq!(cpu.pc, 0x0003);

        opcode = cpu.fetch(instructions.as_slice());
        assert_eq!(opcode, 0xDD);
        assert_eq!(cpu.pc, 0x0004);
    }

}
