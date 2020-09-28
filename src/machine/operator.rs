use super::super::Machine;
use super::register::{RegisterPairs};
use intel8080disassembler;

impl Machine {
  pub fn process_cycles(&mut self) {
    self.check_interrupt();
    let (ins, _) = intel8080disassembler::disassemble(self.registers.pc as usize, self.memory);
    if cfg!(debug_assertions) {
      println!("{:x?}, pc: {:#x}", ins, self.registers.pc);
    }
    self.execute(&ins);
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
        let result = ((*self.locator(ins.operand1)) as u16) + 1;
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
      0xc6 => {
        let result = (self.registers.a as u16) + ins.operand1 as u16;
        self.registers.set_flag(result as u16);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      },
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
      0xce => {
        let result = (self.registers.a as u16) + (ins.operand1 as u16) + (self.registers.c() as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      },
      0xd2 => {
        if self.registers.c() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xd6 => {
        let op = (!ins.operand1 as u16) + 1;
        let result = self.registers.a as u16 + op;
        self.registers.set_sub_flat(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      },
      0xda => {
        if self.registers.c() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xde => {
        // TODO: Maybe a bug here
        let op = (!(ins.operand1 as u16 + self.registers.c() as u16) & 0xff) + 1;
        let result = self.registers.a as u16 + op;
        self.registers.set_sub_flat(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      },
      0xe2 => {
        if self.registers.p() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xe6 => {
        self.registers.a &= ins.operand1;
        self.registers.set_flag(self.registers.a as u16);
        self.registers.pc += 2;
      },
      0xea => {
        if self.registers.p() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xee => {
        let result = self.registers.a ^ ins.operand1;
        self.registers.set_flag(result as u16);
        self.registers.a = result;
        self.registers.pc += 2;
      },
      0xf2 => {
        if self.registers.s() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xf6 => {
        let result = self.registers.a | ins.operand1;
        self.registers.set_flag(result as u16);
        self.registers.a = result;
        self.registers.pc += 2;
      },
      0xfa => {
        if self.registers.s() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      },
      0xfe => {
        let op = (!ins.operand1) as u16 + 1;
        let result = self.registers.a as u16 + op;
        self.registers.set_sub_flat(result);
        self.registers.pc += 2;
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
  fn it_executes_0xc6() {
    let mut machine = Machine::new();
    let ani = intel8080disassembler::Instruction{
      opcode: 0xe6,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&ani);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xc6,
      operand1: 0x06,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 6);
    assert_eq!(machine.registers.c(), 0);
    assert_eq!(machine.registers.p(), 1);
    assert_eq!(machine.registers.s(), 0);
    assert_eq!(machine.registers.z(), 0);
    assert_eq!(machine.registers.pc, 0x04);
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
  fn it_executes_0xce() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xce,
      operand1: 0xab,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xab);
    assert_eq!(machine.registers.pc, 2);
    machine.registers.set_flag(0xfff);
    machine.registers.a = 0x03;
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xab + 4);
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
  fn it_executes_0xd6() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xd6,
      operand1: 0x01,
      operand2: 0x00,
    };
    machine.registers.set_flag(0x0);
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xff);
    assert_eq!(machine.registers.c(), 1);
    assert_eq!(machine.registers.pc, 2);
    machine.registers.a = 0xff;
    let operate = intel8080disassembler::Instruction{
      opcode: 0xd6,
      operand1: 0x22,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xdd);
    assert_eq!(machine.registers.c(), 0)
  }

  #[test]
  fn it_executes_0xda() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xda,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x03);
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x7856);
  }

  #[test]
  fn it_executes_0xde() {
    let mut machine = Machine::new();
    machine.registers.set_flag(0x0);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xde,
      operand1: 0x01,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 2);
    assert_eq!(machine.registers.a, 0xff);
    assert_eq!(machine.registers.c(), 0x1);
    machine.registers.set_flag(0xfff);
    machine.registers.a = 0;
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xfe);
    assert_eq!(machine.registers.c(), 0x1);
  }

  #[test]
  fn it_executes_0xe2() {
    let mut machine = Machine::new();
    machine.registers.set_flag(0x0);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xe2,
      operand1: 0x34,
      operand2: 0x45,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x3);
    machine.registers.set_flag(0b111);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x4534);
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
  fn it_executes_0xee() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xee,
      operand1: 0x81,
      operand2: 0x00,
    };
    machine.registers.a = 0x3b;
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 2);
    assert_eq!(machine.registers.a, 0xba);
  }

  #[test]
  fn it_executes_0xf2() {
    let mut machine = Machine::new();
    machine.registers.set_flag(0b10000000);
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

  #[test]
  fn it_executes_0xf6() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xf6,
      operand1: 0x0f,
      operand2: 0x00,
    };
    machine.registers.a = 0xb5;
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 2);
    assert_eq!(machine.registers.a, 0xbf);
  }

  #[test]
  fn it_executes_0xfa() {
    let mut machine = Machine::new();
    let operate = intel8080disassembler::Instruction{
      opcode: 0xfa,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x3);
    machine.registers.set_flag(0b10000000);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
  }

  #[test]
  fn it_executes_0xfe() {
    let mut machine = Machine::new();
    machine.registers.a = 0x4a;
    let operate = intel8080disassembler::Instruction{
      opcode: 0xfe,
      operand1: 0x4a,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.c(), 0);
    assert_eq!(machine.registers.z(), 1);
    let operate = intel8080disassembler::Instruction{
      opcode: 0xfe,
      operand1: 0xff,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.c(), 1);
    assert_eq!(machine.registers.z(), 0);
  }
}