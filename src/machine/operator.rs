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
      0xf5 => {

      }
      _ => panic!("Not Implemented"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
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
    assert_eq!(machine.flags.c(), 0);
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x00);
    assert_eq!(machine.flags.c(), 1);
  }

  #[test]
  fn it_executes_0xc3() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xc3,
      operand1: 0x12,
      operand2: 0x34,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x1234);
  }
}