use super::super::Machine;
use super::register::{RegisterPairs};
use intel8080disassembler;

impl Machine {
  pub fn process_cycles(&mut self) {
    let (ins, _) = intel8080disassembler::disassemble(self.registers.pc as usize, self.memory);
    println!("{:x?}, pc: {:#x}", ins, self.registers.pc);
    self.execute(&ins);
    // self.registers.pc += pc_increase as u16;
  }

  pub fn execute(&mut self, ins: &intel8080disassembler::Instruction) {
    match ins.opcode {
      0x00 => {
        self.registers.pc += 1;
      }
      0x01 => {
        self.registers.set_rp(RegisterPairs::B, ((ins.operand2 as u16) << 8) + (ins.operand1 as u16));
        self.registers.pc += 1;
      }
      0x03 => {},
      0x04 => {
        let result = ((*self.locator(ins.operand1)) as i16) + 1;
        *self.locator(ins.operand1) = result as u8;
        self.registers.set_flag(result);
        self.registers.pc += 1;
      }
      0x06 => {}
      0x31 => {
        self.registers.sp = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
        self.registers.pc += 3;
      },
      0xc2 => {
        if self.registers.z() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xc3 => {
        self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
      }
      0xca => {
        if self.registers.z() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xcd => {
        self.memory[self.registers.sp as usize - 1] = ((self.registers.pc & 0xff00) >> 8) as u8;
        self.memory[self.registers.sp as usize - 2] = (self.registers.pc & 0xff) as u8;
        self.registers.sp -= 2;
        self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
      }
      0xd2 => {
        if self.registers.c() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xe6 => {
        self.registers.a &= ins.operand1;
        self.registers.set_flag(self.registers.a as i16);
        self.registers.pc += 2;
      },
      0xea => {
        if self.registers.p() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xf2 => {
        if self.registers.s() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      // 0xf5 => {

      // }
      _ => {
        println!("{:x?}", ins);
        println!("PC: {:#x}", self.registers.pc);
        panic!("Not Implemented");
      },
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
      operand1: 0xab,
      operand2: 0x01,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x01ab);
  }

  #[test]
  fn it_executes_0xca() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xca,
      operand1: 0xab,
      operand2: 0x01,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x03);
    machine.registers.set_flag(0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x1ab);
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
  #[test]
  fn it_executes_0xd2() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xd2,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x03);
    machine.registers.set_flag(0x00);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x7856);
  }

  #[test]
  fn it_executes_0xe6() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xe6,
      operand1: 0x10,
      operand2: 0x0,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0x0);
    machine.registers.a = 0x11;
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0x10 & 0x11);
    assert_eq!(machine.registers.pc, 4)
  }

  #[test]
  fn it_executes_0xea() {
    let mut machine = Machine::new();
    machine.registers.set_flag(0b1);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xea,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0b11);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
  }

  #[test]
  fn it_executes_0xf2() {
    let mut machine = Machine::new();
    machine.registers.set_flag(-1);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xf2,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0b1);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
  }
}