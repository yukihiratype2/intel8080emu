use super::super::Machine;
use super::register::RegisterPairs;
use intel8080disassembler;

impl<'a> Machine<'a> {
  pub fn process_cycles(&mut self) {
    self.check_interrupt();
    let (ins, _) = intel8080disassembler::disassemble(self.registers.pc as usize, self.memory);
    self.execute(&ins);
    if cfg!(debug_assertions) {
      println!("{:x?}, pc: {:#x}, a: {:#x}", ins, self.registers.pc, self.registers.a);
    }
  }

  pub fn execute(&mut self, ins: &intel8080disassembler::Instruction) {
    match ins.opcode {
      0x00 => {
        self.registers.pc += 1;
      }
      0x01 => {
        self.registers.set_rp(
          RegisterPairs::B,
          ((ins.operand2 as u16) << 8) + (ins.operand1 as u16),
        );
        self.registers.pc += 3;
      }
      0x04 => {
        let result = (self.registers.b as u16) + 0x01;
        self.registers.set_flag(result);
        self.registers.b = (result & 0xff) as u8;
        self.registers.pc += 1;
      }
      0x05 => {
        let result = (self.registers.b as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.b = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x06 => {
        self.registers.b = ins.operand1;
        self.registers.pc += 2;
      },
      0x09 => {
        let result = self.registers.get_rp(RegisterPairs::H) as u32 + self.registers.get_rp(RegisterPairs::B) as u32;
        self.registers.set_flag_cy_16(result);
        self.registers.set_rp(RegisterPairs::H, (result & 0xffff) as u16);
        self.registers.pc += 1;
      },
      0x0c => {
        // Not tested
        let result = (self.registers.c as u16) + 0x01;
        self.registers.set_flag(result);
        self.registers.c = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x0d => {
        let result = (self.registers.c as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.c = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x0e => {
        self.registers.c = ins.operand1;
        self.registers.pc += 2;
      },
      0x0f => {
        // Not tested
        let a0 = self.registers.a & 0b1;
        self.registers.flag &= 0b11111110;
        self.registers.flag |= a0;
        self.registers.a = self.registers.a >> 1;
        self.registers.a |= a0 << 7;
        self.registers.pc += 1;
      }
      0x11 => {
        self.registers.set_rp(
          RegisterPairs::D,
          ((ins.operand2 as u16) << 8) + (ins.operand1 as u16),
        );
        self.registers.pc += 3;
      }
      0x1a => {
        let location = self.registers.get_rp(RegisterPairs::D);
        self.registers.a = self.memory[location as usize];
        self.registers.pc += 1;
      },
      0x1f => {
        let c = self.registers.c();
        let a0 = self.registers.a & 0b1;
        self.registers.flag &= 0b11111110;
        self.registers.flag |= a0;
        self.registers.a = self.registers.a >> 1;
        self.registers.a |= c << 7;
        self.registers.pc +=1;
      },
      0x13 => {
        let result = self.registers.get_rp(RegisterPairs::D) as u32 + 1;
        self.registers.set_rp(RegisterPairs::D, (result & 0xffff) as u16);
        self.registers.pc += 1;
      }
      0x14 => {
        // May have bug
        let result = (self.registers.d as u16) + 1;
        self.registers.set_flag(result);
        self.registers.d = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x15 => {
        let result = (self.registers.d as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.d = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x16 => {
        self.registers.d = ins.operand1;
        self.registers.pc += 2;
      },
      0x1c => {
        let result = (self.registers.e as u16) + 0x01;
        self.registers.set_flag(result);
        self.registers.e = (result & 0xff) as u8;
        self.registers.pc += 1;
      }
      0x1d => {
        let result = (self.registers.e as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.e = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x19 => {
        let result = self.registers.get_rp(RegisterPairs::H) as u32 + self.registers.get_rp(RegisterPairs::D) as u32;
        self.registers.set_flag_cy_16(result);
        self.registers.set_rp(RegisterPairs::H, (result & 0xffff) as u16);
        self.registers.pc += 1;
      },
      0x1e => {
        self.registers.e = ins.operand1;
        self.registers.pc += 2;
      },
      0x21 => {
        // Not tested
        let result = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
        self.registers.set_rp(RegisterPairs::H, result);
        self.registers.pc += 3;
      }
      0x23 => {
        let result = self.registers.get_rp(RegisterPairs::H) as u32 + 1;
        self.registers.set_rp(RegisterPairs::H, (result & 0xffff) as u16);
        self.registers.pc += 1;
      }
      0x24 => {
        // Not tested
        let result = (self.registers.h as u16) + 0x01;
        self.registers.set_flag(result);
        self.registers.h = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x25 => {
        let result = (self.registers.h as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.h = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x26 => {
        self.registers.h = ins.operand1;
        self.registers.pc += 2;
      },
      0x29 => {
        let result = self.registers.get_rp(RegisterPairs::H) as u32 * 2;
        self.registers.set_flag_cy_16(result);
        self.registers.set_rp(RegisterPairs::H, (result & 0xffff) as u16);
        self.registers.pc += 1;
      },
      0x2c => {
        // Not tested
        let result = (self.registers.l as u16) + 0x01;
        self.registers.set_flag(result);
        self.registers.l = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x2d => {
        let result = (self.registers.l as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.l = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x2e => {
        self.registers.l = ins.operand1;
        self.registers.pc += 2;
      },
      0x31 => {
        self.registers.sp = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
        self.registers.pc += 3;
      }
      0x32 => {
        // Not tested
        let addr = ((ins.operand2 as u16) << 8 ) + (ins.operand1 as u16);
        self.memory[addr as usize] = self.registers.a;
        self.registers.pc += 3;
      },
      0x35 => {
        // Not tested
        let result = (self.memory[self.registers.get_rp(RegisterPairs::H) as usize] + 100 - 1) & 0xff;
        self.registers.set_sub_flat(result as u16);
        self.registers.pc += 1;
      },
      0x36 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = ins.operand1;
        self.registers.pc += 2;
      },
      0x3a => {
        // Not tested
        let addr = ((ins.operand2 as u16) << 8 ) + (ins.operand1 as u16);
        self.registers.a = self.memory[addr as usize];
        self.registers.pc += 3;
      },
      0x3d => {
        let result = (self.registers.a as u16) + 0xff;
        self.registers.set_sub_flat(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x3e => {
        self.registers.a = ins.operand1;
        self.registers.pc += 2;
      }
      0x3c => {
        let result = (self.registers.a as u16) + 1;
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x41 => {
        self.registers.b = self.registers.c;
        self.registers.pc += 1;
      },
      0x42 => {
        self.registers.b = self.registers.d;
        self.registers.pc += 1;
      },
      0x43 => {
        self.registers.b = self.registers.e;
        self.registers.pc += 1;
      },
      0x44 => {
        self.registers.b = self.registers.h;
        self.registers.pc += 1;
      },
      0x45 => {
        self.registers.b = self.registers.l;
        self.registers.pc += 1;
      },
      0x46 => {
        self.registers.b = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x47 => {
        self.registers.b = self.registers.a;
        self.registers.pc += 1;
      },
      0x48 => {
        self.registers.c = self.registers.b;
        self.registers.pc += 1;
      },
      0x49 => {
        self.registers.c = self.registers.c;
        self.registers.pc += 1;
      },
      0x4a => {
        self.registers.c = self.registers.d;
        self.registers.pc += 1;
      },
      0x4b => {
        self.registers.c = self.registers.e;
        self.registers.pc += 1;
      },
      0x4c => {
        self.registers.c = self.registers.h;
        self.registers.pc += 1;
      },
      0x4d => {
        self.registers.c = self.registers.l;
        self.registers.pc += 1;
      },
      0x4e => {
        self.registers.c = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x4f => {
        self.registers.c = self.registers.a;
        self.registers.pc += 1;
      },
      0x50 => {
        self.registers.d = self.registers.b;
        self.registers.pc += 1;
      },
      0x51 => {
        self.registers.d = self.registers.c;
        self.registers.pc += 1;
      },
      0x52 => {
        self.registers.d = self.registers.d;
        self.registers.pc += 1;
      },
      0x53 => {
        self.registers.d = self.registers.e;
        self.registers.pc += 1;
      },
      0x54 => {
        self.registers.d = self.registers.h;
        self.registers.pc += 1;
      },
      0x55 => {
        self.registers.d = self.registers.l;
        self.registers.pc += 1;
      },
      0x56 => {
        self.registers.d = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x57 => {
        self.registers.d = self.registers.a;
        self.registers.pc += 1;
      },
      0x58 => {
        self.registers.e = self.registers.b;
        self.registers.pc += 1;
      },
      0x59 => {
        self.registers.e = self.registers.c;
        self.registers.pc += 1;
      },
      0x5a => {
        self.registers.e = self.registers.d;
        self.registers.pc += 1;
      },
      0x5c => {
        self.registers.e = self.registers.h;
        self.registers.pc += 1;
      },
      0x5d => {
        self.registers.e = self.registers.l;
        self.registers.pc += 1;
      },
      0x5e => {
        self.registers.e = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x5f => {
        self.registers.e = self.registers.a;
        self.registers.pc += 1;
      },
      0x60 => {
        self.registers.h = self.registers.b;
        self.registers.pc += 1;
      },
      0x61 => {
        self.registers.h = self.registers.c;
        self.registers.pc += 1;
      },
      0x62 => {
        self.registers.h = self.registers.d;
        self.registers.pc += 1;
      },
      0x63 => {
        self.registers.h = self.registers.e;
        self.registers.pc += 1;
      },
      0x65 => {
        self.registers.h = self.registers.l;
        self.registers.pc += 1;
      },
      0x66 => {
        self.registers.h = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x67 => {
        self.registers.h = self.registers.a;
        self.registers.pc += 1;
      },
      0x68 => {
        self.registers.l = self.registers.b;
        self.registers.pc += 1;
      },
      0x69 => {
        self.registers.l = self.registers.c;
        self.registers.pc += 1;
      },
      0x6a => {
        self.registers.l = self.registers.d;
        self.registers.pc += 1;
      },
      0x6b => {
        self.registers.l = self.registers.e;
        self.registers.pc += 1;
      },
      0x6c => {
        self.registers.l = self.registers.h;
        self.registers.pc += 1;
      },
      0x6d => {
        self.registers.l = self.registers.l;
        self.registers.pc += 1;
      },
      0x6e => {
        self.registers.l = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x6f => {
        self.registers.l = self.registers.a;
        self.registers.pc += 1;
      },
      0x70 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.b;
        self.registers.pc += 1;
      },
      0x71 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.c;
        self.registers.pc += 1;
      },
      0x72 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.d;
        self.registers.pc += 1;
      },
      0x73 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.e;
        self.registers.pc += 1;
      },
      0x74 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.h;
        self.registers.pc += 1;
      },
      0x75 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.l;
        self.registers.pc += 1;
      },
      0x77 => {
        self.memory[self.registers.get_rp(RegisterPairs::H) as usize] = self.registers.a;
        self.registers.pc += 1;
      },
      0x78 => {
        self.registers.a = self.registers.b;
        self.registers.pc += 1;
      },
      0x79 => {
        self.registers.a = self.registers.c;
        self.registers.pc += 1;
      },
      0x7a => {
        self.registers.a = self.registers.d;
        self.registers.pc += 1;
      },
      0x7b => {
        self.registers.a = self.registers.e;
        self.registers.pc += 1;
      },
      0x7c => {
        self.registers.a = self.registers.h;
        self.registers.pc += 1;
      },
      0x7d => {
        self.registers.a = self.registers.l;
        self.registers.pc += 1;
      },
      0x7e => {
        self.registers.a = self.memory[self.registers.get_rp(RegisterPairs::H) as usize];
        self.registers.pc += 1;
      },
      0x80 => {
        let result = (self.registers.a as u16) + (self.registers.b as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x81 => {
        let result = (self.registers.a as u16) + (self.registers.c as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x82 => {
        let result = (self.registers.a as u16) + (self.registers.d as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x83 => {
        let result = (self.registers.a as u16) + (self.registers.e as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x84 => {
        let result = (self.registers.a as u16) + (self.registers.h as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x85 => {
        let result = (self.registers.a as u16) + (self.registers.l as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0x87 => {
        let result = (self.registers.a as u16) + (self.registers.a as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 1;
      },
      0xa7 => {
        let result = self.registers.a & self.registers.a;
        self.registers.set_flag(result as u16);
        self.registers.pc += 1;
      },
      0xaf => {
        self.registers.a = 0;
        self.registers.set_flag(0);
        self.registers.pc += 1;
      },
      0xc0 => {
        if self.registers.z() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      },
      0xc1 => {
        self.registers.c = self.memory[self.registers.sp as usize];
        self.registers.b = self.memory[(self.registers.sp as usize) + 1];
        self.registers.sp += 2;
        self.registers.pc += 1;
      },
      0xc2 => {
        if self.registers.z() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xc3 => {
        self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
      }
      0xc4 => {
        if self.registers.z() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xc5 => {
        self.memory[self.registers.sp as usize - 2] = self.registers.c;
        self.memory[self.registers.sp as usize - 1] = self.registers.b;
        self.registers.sp -= 2;
        self.registers.pc += 1;
      },
      0xc6 => {
        let result = (self.registers.a as u16) + ins.operand1 as u16;
        self.registers.set_flag(result as u16);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      }
      0xc8 => {
        if self.registers.z() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      },
      0xc9 => {
        self.registers.pc = (self.memory[self.registers.sp as usize] as u16)
          + ((self.memory[self.registers.sp as usize + 1] as u16) << 8);
        self.registers.sp += 2;
      }
      0xca => {
        if self.registers.z() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xcc => {
        if self.registers.z() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xcd => {
        self.memory[self.registers.sp as usize - 1] = (((self.registers.pc + 3) & 0xff00) >> 8) as u8;
        self.memory[self.registers.sp as usize - 2] = ((self.registers.pc + 3) & 0xff) as u8;
        self.registers.sp -= 2;
        self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
      }
      0xce => {
        let result =
          (self.registers.a as u16) + (ins.operand1 as u16) + (self.registers.c() as u16);
        self.registers.set_flag(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      }
      0xd0 => {
        if self.registers.c() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xd1 => {
        self.registers.e = self.memory[self.registers.sp as usize];
        self.registers.d = self.memory[(self.registers.sp as usize) + 1];
        self.registers.sp += 2;
        self.registers.pc += 1;
      },
      0xd2 => {
        if self.registers.c() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xd3 => {
        self.pin.ports[ins.operand1 as usize](self.registers.a);
        self.registers.pc += 2;
      }
      0xd4 => {
        if self.registers.c() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xd5 => {
        self.memory[self.registers.sp as usize - 2] = self.registers.e;
        self.memory[self.registers.sp as usize - 1] = self.registers.d;
        self.registers.sp -= 2;
        self.registers.pc += 1;
      },
      0xd6 => {
        let op = (!ins.operand1 as u16) + 1;
        let result = self.registers.a as u16 + op;
        self.registers.set_sub_flat(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      }
      0xd8 => {
        if self.registers.c() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xda => {
        if self.registers.c() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xdb => {
        self.registers.a = self.pin.ports[ins.operand1 as usize](0);
        self.registers.pc += 2;
      }
      0xdc => {
        if self.registers.c() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xde => {
        // TODO: Maybe a bug here
        let op = (!(ins.operand1 as u16 + self.registers.c() as u16) & 0xff) + 1;
        let result = self.registers.a as u16 + op;
        self.registers.set_sub_flat(result);
        self.registers.a = (result & 0xff) as u8;
        self.registers.pc += 2;
      }
      0xe0 => {
        if self.registers.p() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xe1 => {
        self.registers.l = self.memory[self.registers.sp as usize];
        self.registers.h = self.memory[(self.registers.sp as usize) + 1];
        self.registers.sp += 2;
        self.registers.pc += 1;
      },
      0xe2 => {
        if self.registers.p() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xe4 => {
        if self.registers.p() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      },
      0xe5 => {
        self.memory[self.registers.sp as usize - 2] = self.registers.l;
        self.memory[self.registers.sp as usize - 1] = self.registers.h;
        self.registers.sp -= 2;
        self.registers.pc += 1;
      },
      0xe6 => {
        self.registers.a &= ins.operand1;
        self.registers.set_flag(self.registers.a as u16);
        self.registers.pc += 2;
      }
      0xe8 => {
        if self.registers.p() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xea => {
        if self.registers.p() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xeb => {
        // swap d
        let d = self.registers.d;
        self.registers.d = self.registers.h;
        self.registers.h = d;
        // swap e
        let e = self.registers.e;
        self.registers.e = self.registers.l;
        self.registers.l = e;
        self.registers.pc += 1;
      },
      0xec => {
        if self.registers.p() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xee => {
        let result = self.registers.a ^ ins.operand1;
        self.registers.set_flag(result as u16);
        self.registers.a = result;
        self.registers.pc += 2;
      }
      0xf0 => {
        if self.registers.s() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xf1 => {
        self.registers.flag = self.memory[self.registers.sp as usize];
        self.registers.a = self.memory[(self.registers.sp as usize) + 1];
        self.registers.sp += 2;
        self.registers.pc += 1;
      },
      0xf2 => {
        if self.registers.s() == 0 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xf4 => {
        if self.registers.s() == 0 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
      0xf5 => {
        self.memory[self.registers.sp as usize - 2] = self.registers.flag;
        self.memory[self.registers.sp as usize - 1] = self.registers.a;
        self.registers.sp -= 2;
        self.registers.pc += 1;
      },
      0xf6 => {
        let result = self.registers.a | ins.operand1;
        self.registers.set_flag(result as u16);
        self.registers.a = result;
        self.registers.pc += 2;
      }
      0xf8 => {
        if self.registers.s() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xc9,
            operand1: 0x00,
            operand2: 0x00,
          });
          return;
        }
        self.registers.pc += 1;
      }
      0xfa => {
        if self.registers.s() == 1 {
          self.registers.pc = ((ins.operand2 as u16) << 8) + (ins.operand1 as u16);
          return;
        }
        self.registers.pc += 3;
      }
      0xfb => {
        println!("Interrupt ebled");
        self.pin.ei = true;
        self.registers.pc += 1;
      },
      0xfc => {
        if self.registers.s() == 1 {
          self.execute(&intel8080disassembler::Instruction {
            opcode: 0xcd,
            operand1: ins.operand1,
            operand2: ins.operand2,
          });
          return;
        }
        self.registers.pc += 3;
      }
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
        unimplemented!();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_executes_0x01() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x01,
      operand1: 0x12,
      operand2: 0x24,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.b, 0x24);
    assert_eq!(machine.registers.c, 0x12);
  }

  #[test]
  fn it_executes_0x14() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x14,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.d, 1);
    machine.registers.d = 0xff;
    machine.execute(&operate);
    assert_eq!(machine.registers.d, 0);
  }

  #[test]
  fn it_executes_0x1a() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.memory[0xcdef] = 0x12;
    machine.registers.d = 0xcd;
    machine.registers.e = 0xef;
    let operate = intel8080disassembler::Instruction {
      opcode: 0x1a,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.a, 0x12);
  }

  #[test]
  fn it_executes_0x1f() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x1f,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.a = 0b01101010;
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0b10110101);
    assert_eq!(machine.registers.c(), 0);
    assert_eq!(machine.registers.pc, 1);
  }

  #[test]
  fn it_executes_0x31() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x31,
      operand1: 0x12,
      operand2: 0x24,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.sp, 0x2412);
  }

  #[test]
  fn it_executes_0x47() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x47,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.a = 0x12;
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.b, 0x12);
  }

  #[test]
  fn it_executes_0x04() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0x0d() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x0d,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.c = 0xff;
    machine.execute(&operate);
    assert_eq!(machine.registers.c, 0xff - 1);
    assert_eq!(machine.registers.pc, 1);
  }

  #[test]
  fn it_executes_0x23() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x23,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_rp(RegisterPairs::H, 0x1234);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.get_rp(RegisterPairs::H), 0x1235);
    machine.registers.set_rp(RegisterPairs::H, 0xffff);
    machine.execute(&operate);
    assert_eq!(machine.registers.get_rp(RegisterPairs::H), 0x0);
  }

  #[test]
  fn it_executes_0x29() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x29,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_rp(RegisterPairs::H, 0x1234);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.get_rp(RegisterPairs::H), 0x1234 * 2);
    assert_eq!(machine.registers.c(), 0);
    machine.registers.set_rp(RegisterPairs::H, 0xffff);
    machine.execute(&operate);
    assert_eq!(machine.registers.get_rp(RegisterPairs::H), 0xfffe);
    assert_eq!(machine.registers.c(), 1);
  }

  #[test]
  fn it_executes_0x5e() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0x5e,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_rp(RegisterPairs::H, 0x1234);
    machine.memory[0x1234] = 0xaa;
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.e, 0xaa);
  }

  #[test]
  fn it_executes_0x77() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_rp(RegisterPairs::H, 0x1234);
    machine.registers.a = 0x45;
    let operate = intel8080disassembler::Instruction {
      opcode: 0x77,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.memory[0x1234], 0x45);
  }

  #[test]
  fn it_executes_0x80() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.a = 0x45;
    machine.registers.b = 0x55;
    let operate = intel8080disassembler::Instruction {
      opcode: 0x80,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.a, 0x45 + 0x55);
  }

  #[test]
  fn it_executes_0xc0() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xc0,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_flag(0x0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0xff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }
  #[test]
  fn it_executes_0xc3() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0xc3,
      operand1: 0xab,
      operand2: 0x01,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x01ab);
  }

  #[test]
  fn it_executes_0xc4() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xc4,
      operand1: 0xab,
      operand2: 0x01,
    };
    machine.registers.set_flag(0x00);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0x01);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x01ab);
  }

  #[test]
  fn it_executes_0xc6() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
    assert_eq!(machine.registers.pc, 0x02);
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 12);
  }

  #[test]
  fn it_executes_0xc8() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xc8,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0x00);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }
  #[test]
  fn it_executes_0xc9() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0x12;
    machine.memory[0xff + 1] = 0xab;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xc9,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.sp, 0xff + 2);
    assert_eq!(machine.registers.pc, 0xab12);
  }

  #[test]
  fn it_executes_0xca() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xcc() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xcc,
      operand1: 0xab,
      operand2: 0x10,
    };
    machine.registers.set_flag(0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x10ab);
    machine.registers.set_flag(1);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x10ab + 3);
  }

  #[test]
  fn it_executes_0xcd() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xeeee;
    machine.registers.pc = 0x1234;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xcd,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.sp, 0xeeee - 2);
    assert_eq!(machine.memory[0xeeee - 1], 0x12);
    assert_eq!(machine.memory[0xeeee - 2], 0x34 + 3);
    assert_eq!(machine.registers.sp, 0xeeee - 2);
    assert_eq!(machine.registers.pc, 0x7856)
  }

  #[test]
  fn it_executes_0xce() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xd0() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xd0,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0xff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xd2() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xd4() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0xd4,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.registers.sp = 0xff;
    machine.registers.set_flag(0x0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x7856);
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x7856 + 3);
  }

  #[test]
  fn it_executes_0xd5() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xd5,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_rp(RegisterPairs::D, 0xabcd);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.memory[0xff - 2], 0xcd);
    assert_eq!(machine.memory[0xff - 1], 0xab);
    assert_eq!(machine.registers.sp, 0xff - 2);
  }

  #[test]
  fn it_executes_0xd6() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
    let operate = intel8080disassembler::Instruction {
      opcode: 0xd6,
      operand1: 0x22,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.a, 0xdd);
    assert_eq!(machine.registers.c(), 0)
  }

  #[test]
  fn it_executes_0xd8() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xd8,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_flag(0b00);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xda() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xdc() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0xdc,
      operand1: 0x56,
      operand2: 0x78,
    };
    machine.registers.sp = 0xff;
    machine.execute(&operate);
    machine.registers.set_flag(0x0);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0xfff);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x7856);
  }

  #[test]
  fn it_executes_0xde() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0x0);
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xe0() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xe0,
      operand1: 0x0,
      operand2: 0x0,
    };
    machine.registers.set_flag(0b0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0b111);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xe1() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0x12;
    machine.memory[0xff + 1] = 0x23;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xe1,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.l, 0x12);
    assert_eq!(machine.registers.h, 0x23);
    assert_eq!(machine.registers.sp, 0xff + 2);
  }

  #[test]
  fn it_executes_0xe2() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0x0);
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xe4() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0x0);
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xe4,
      operand1: 0x34,
      operand2: 0x45,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0b111);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0x4534);
  }

  #[test]
  fn it_executes_0xe6() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xe8() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xe8,
      operand1: 0x0,
      operand2: 0x0,
    };
    machine.registers.set_flag(0b1);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0b1111);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xea() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0b1);
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xeb() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
      opcode: 0xeb,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_rp(RegisterPairs::D, 0xabcd);
    machine.registers.set_rp(RegisterPairs::H, 0x1234);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    assert_eq!(machine.registers.get_rp(RegisterPairs::H), 0xabcd);
    assert_eq!(machine.registers.get_rp(RegisterPairs::D), 0x1234);
  }

  #[test]
  fn it_executes_0xec() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xec,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.registers.set_flag(0x11);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
    machine.registers.set_flag(0x1);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23 + 3);
  }

  #[test]
  fn it_executes_0xee() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0b10000000);
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xf0() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xf0,
      operand1: 0x0,
      operand2: 0x0,
    };
    machine.registers.set_flag(0b10000000);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0b0011);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xf4() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.set_flag(0b10000000);
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xf4,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0x0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
  }

  #[test]
  fn it_executes_0xf6() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xf8() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    machine.memory[0xff] = 0xab;
    machine.memory[0xff + 1] = 0xcd;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xf8,
      operand1: 0x00,
      operand2: 0x00,
    };
    machine.registers.set_flag(0b00);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 1);
    machine.registers.set_flag(0b10001111);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xcdab);
  }

  #[test]
  fn it_executes_0xfa() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    let operate = intel8080disassembler::Instruction {
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
  fn it_executes_0xfc() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.sp = 0xff;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xfc,
      operand1: 0x23,
      operand2: 0xac,
    };
    machine.registers.set_flag(0x0);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 3);
    machine.registers.set_flag(0b10000000);
    machine.execute(&operate);
    assert_eq!(machine.registers.pc, 0xac23);
  }

  #[test]
  fn it_executes_0xfe() {
    let mut machine = Machine::new(vec!(&|x| {x}));
    machine.registers.a = 0x4a;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xfe,
      operand1: 0x4a,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.c(), 0);
    assert_eq!(machine.registers.z(), 1);
    let operate = intel8080disassembler::Instruction {
      opcode: 0xfe,
      operand1: 0xff,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.c(), 1);
    assert_eq!(machine.registers.z(), 0);
    machine.registers.a = 0x47;
    let operate = intel8080disassembler::Instruction {
      opcode: 0xfe,
      operand1: 0x47,
      operand2: 0x00,
    };
    machine.execute(&operate);
    assert_eq!(machine.registers.z(), 1);
    assert_eq!(machine.registers.c(), 0);
    assert_eq!(machine.registers.p(), 1);
    assert_eq!(machine.registers.s(), 0);
    assert_eq!(machine.registers.a, 0x47);
  }
}
