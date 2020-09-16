pub mod flags;
pub mod registers;
pub mod memory;
use intel8080disassembler;

pub struct Machine {
  pub flags: flags::Flags,
  pub registers: registers::Registers,
  pub memory: memory::Memory,
}

impl Machine {
  pub fn new() -> Machine {
    return Machine{
      flags: flags::Flags::new(),
      registers: registers::Registers::new(),
      memory: [0; 65535]
    }
  }


  pub fn excute(&mut self, ins: intel8080disassembler::Instruction) {
    match ins.opcode {
      0x03 => {},
      0x04 => {
        let result = ((*self.locator(ins.operand1)) as u16) + 1;
        *self.locator(ins.operand1) = result as u8;
        self.flags.set_flag(result)
      }
      _ => panic!("Not Implemented"),
    }
  }

  fn locator(&mut self, reg: u8) -> &mut u8 {
    match reg {
      0b111 => {
        return &mut self.registers.a
      }
      0b000 => {
        return &mut self.registers.b
      }
      0b110 => {
        let loc = (self.registers.h as usize) << 8 + self.registers.l as usize;
        return &mut self.memory[loc]
      }
      _ => {panic!("Not Implemented")}
    }
  }
}