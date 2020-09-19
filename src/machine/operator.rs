use super::super::Machine;
use intel8080disassembler;

impl Machine {
  pub fn execute(&mut self, ins: &intel8080disassembler::Instruction) {
    match ins.opcode {
      0x03 => {},
      0x04 => {
        let result = ((*self.locator(ins.operand1)) as u16) + 1;
        *self.locator(ins.operand1) = result as u8;
        self.flags.set_flag(result);
        self.registers.pc += 1;
      }
      0xc3 => {
        self.registers.pc = ((ins.operand1 as u16) << 8) + (ins.operand2 as u16);
      }
      _ => panic!("Not Implemented"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_executes_0x04() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0x04,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x01);
    machine.registers.b = 0xff;
    assert_eq!(machine.flags.c, false);
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x00);
    assert_eq!(machine.flags.c, true);
  }
}