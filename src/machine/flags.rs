pub struct Flags {
  flag: u8,
}

impl Flags {
  pub fn new() -> Flags {
    return Flags {
      flag: 0,
    }
  }

  pub fn set_flag(&mut self, result: u16) {
    if result == 0 {
      self.flag |= 0x40;
    }
    if result > 0xff {
      self.flag |= 0x01;
    }
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
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_sets_flag_z() {
    let mut flags = Flags::new();
    flags.set_flag(1);
    assert_eq!(flags.z(), 0);
    flags.set_flag(0);
    assert_eq!(flags.z(), 1);
  }

  #[test]
  #[ignore]
  //TODO: confirm flag works correctly
  fn it_sets_flag_c() {
    let mut flags = Flags::new();
    flags.set_flag(0);
    assert_eq!(flags.c(), 0);
    flags.set_flag(0xff);
    assert_eq!(flags.c(), 0);
    flags.set_flag(0xff + 1);
    assert_eq!(flags.c(), 1);
  }
}