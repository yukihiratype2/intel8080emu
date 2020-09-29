// pub enum GenericRegister {
//   B,
//   C,
//   D,
//   E,
//   H,
//   L
// }
pub enum RegisterPairs {
  B,
  D,
  H,
  PSW
}
pub struct Registers {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub pc: u16,
  pub sp: u16,
  pub flag: u8,
}

impl Registers {
  pub fn new() -> Registers {
    return Registers {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      pc: 0,
      sp: 0,
      flag: 0b00000010,
    }
  }

  pub fn set_flag(&mut self, result: u16) {
    self.flag = if (result & 0xff) == 0 { self.flag | 0b01000000 } else { self.flag & 0b10111111 };
    self.flag = if result > 0xff { self.flag | 0b01 } else { self.flag & 0b11111110 };
    let mut x = result & 0xff;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    x = ((!x) & 1) << 2;
    self.flag = if x != 0 { self.flag | 0b100 } else {self.flag & 0b11111011};
    self.flag = if (result & 0b10000000) == 0b10000000 { self.flag | 0b10000000 } else  {self.flag & 0b01111111};
  }
  pub fn set_sub_flat(&mut self, result: u16) {
    self.set_flag(result);
    self.flag ^= 0b1;
    self.flag = if (result & 0xff) == 0 { self.flag | 0b01000000 } else { self.flag & 0b10111111 };
  }

  pub fn c(&self) -> u8 {
    return self.flag & 0b1;
  }
  pub fn ac(&self) -> u8 {
    return (self.flag & 0x10) >> 4;
  }
  pub fn s(&self) -> u8 {
    return (self.flag & 0x80) >> 7;
  }
  pub fn z(&self) -> u8 {
    return (self.flag & 0x40) >> 6;
  }
  pub fn p(&self) -> u8 {
    return (self.flag & 0x04) >> 2;
  }

  // get register pair (16-bit)
  pub fn get_rp(&self, r: RegisterPairs) -> u16 {
    match r {
      RegisterPairs::B => {
        return ((self.b as u16) << 8) + self.c as u16;
      }
      RegisterPairs::D => {
        return ((self.d as u16) << 8) + self.e as u16;
      }
      RegisterPairs::H => {
        return ((self.h as u16) << 8) + self.l as u16;
      }
      RegisterPairs::PSW => {
        return ((self.a as u16) << 8) + self.flag as u16;
      }
    }
  }

  pub fn set_rp(&mut self, r: RegisterPairs, v:u16) {
    match r {
      RegisterPairs::B => {
        self.b = (v >> 8) as u8;
        self.c = (v & 0xff) as u8;
      }
      RegisterPairs::D => {
        self.d = (v >> 8) as u8;
        self.e = (v & 0xff) as u8;
      }
      RegisterPairs::H => {
        self.h = (v >> 8) as u8;
        self.l = (v & 0xff) as u8;
      }
      RegisterPairs::PSW => {
        panic!("Can't set PSW")
      }
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_get_rp() {
    let mut registers = Registers::new();
    registers.b = 0xff;
    registers.c = 0xff;
    assert_eq!(registers.get_rp(RegisterPairs::B), 0xffff);
    assert_eq!(registers.get_rp(RegisterPairs::D), 0x0000);
    registers.h = 0x12;
    registers.l = 0x34;
    assert_eq!(registers.get_rp(RegisterPairs::H), 0x1234);
  }

  #[test]
  fn it_set_rp() {
    let mut registers = Registers::new();
    registers.set_rp(RegisterPairs::B, 0xffff);
    assert_eq!(registers.b, 0xff);
    assert_eq!(registers.c, 0xff);
    registers.set_rp(RegisterPairs::B, 0x1234);
    assert_eq!(registers.b, 0x12);
    assert_eq!(registers.c, 0x34);
    registers.set_rp(RegisterPairs::H, 0x1234);
    assert_eq!(registers.h, 0x12);
    assert_eq!(registers.l, 0x34);
  }

  #[test]
  fn it_sets_flag_z() {
    let mut flags = Registers::new();
    flags.set_flag(1);
    assert_eq!(flags.z(), 0);
    flags.set_flag(0);
    assert_eq!(flags.z(), 1);
    flags.set_flag(1);
    assert_eq!(flags.z(), 0);
  }

  #[test]
  fn it_sets_flag_c() {
    let mut flags = Registers::new();
    flags.set_flag(0);
    assert_eq!(flags.c(), 0);
    flags.set_flag(0xff);
    assert_eq!(flags.c(), 0);
    flags.set_flag(0xff + 1);
    assert_eq!(flags.c(), 1);
    flags.set_flag(0x00 + 1);
    assert_eq!(flags.c(), 0);
  }

  #[test]
  fn it_sets_flag_p() {
    let mut flags = Registers::new();
    flags.set_flag(0);
    assert_eq!(flags.p(), 1);
    flags.set_flag(1);
    assert_eq!(flags.p(), 0);
    flags.set_flag(0b11);
    assert_eq!(flags.p(), 1);
    flags.set_flag(0b111);
    assert_eq!(flags.p(), 0);
    flags.set_flag(0b1011);
    assert_eq!(flags.p(), 0);
    flags.set_flag(0xf5);
    assert_eq!(flags.p(), 1);
  }

  #[test]
  fn test_set_flag_s() {
    let mut flags = Registers::new();
    flags.set_flag(1);
    assert_eq!(flags.s(), 0);
    flags.set_flag(0b10000000);
    assert_eq!(flags.s(), 1);
    flags.set_flag(0xfff);
    assert_eq!(flags.s(), 1);
  }
}