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
  // PSW
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
    }
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
      // RegisterPairs::PSW => {

      // }
    }
  }

  pub fn set_rp(&mut self, r: RegisterPairs, v:u16) {
    match r {
      RegisterPairs::B => {
        self.b = (v >> 8) as u8;
        self.c = (v & 0xff) as u8;
      }
      RegisterPairs::D => {
        self.b = (v >> 8) as u8;
        self.e = (v & 0xff) as u8;
      }
      RegisterPairs::H => {
        self.h = (v >> 8) as u8;
        self.l = (v & 0xff) as u8;
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
}