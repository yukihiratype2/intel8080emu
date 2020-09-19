use super::super::Machine;
use super::registers::{RegisterPairs};
use intel8080disassembler;

impl Machine {
  pub fn execute(&mut self, ins: &intel8080disassembler::Instruction) {
    match ins.opcode {
      0x01 => {
        self.registers.set_rp(RegisterPairs::B, ((ins.operand2 as u16) << 8) + (ins.operand1 as u16));
      }
      0x03 => {},
      0x04 => {
        let result = ((*self.locator(ins.operand1)) as u16) + 1;
        *self.locator(ins.operand1) = result as u8;
        self.registers.set_flag(result);
        self.registers.pc += 1;
      }
      0x06 => {}
      0x31 => {
        self.registers.sp = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
      },
      0xc3 => {
        self.registers.pc = ((ins.operand1 as u16) << 8) + (ins.operand2 as u16);
      }
      0xcd => {
        self.memory[self.registers.sp as usize - 1] = ((self.registers.pc & 0xff00) >> 8) as u8;
        self.memory[self.registers.sp as usize - 2] = (self.registers.pc & 0xff) as u8;
        self.registers.sp -= 2;
        self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
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
  fn it_executes_0x01() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0x01,
      operand1: 0x12,
      operand2: 0x24,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x24);
    assert_eq!(machine.registers.c, 0x12);
  }

  #[test]
  fn it_executes_0x31() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0x31,
      operand1: 0x12,
      operand2: 0x24,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.sp, 0x2412);
  }

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
    assert_eq!(machine.registers.c(), 0);
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x00);
    assert_eq!(machine.registers.c(), 1);
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

  #[test]
  fn it_executes_0xcd() {
    let mut machine = Machine::new();
    machine.registers.sp = 0xeeee;
    machine.registers.pc = 0x1234;
    let operate = intel8080disassembler::Instruction{
      opcode: 0xcd,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.sp, 0xeeee - 2);
    assert_eq!(machine.memory[0xeeee - 1], 0x12);
    assert_eq!(machine.memory[0xeeee - 2], 0x34);
    assert_eq!(machine.registers.sp, 0xeeee - 2);
    assert_eq!(machine.registers.pc, 0x7856)
  }
}