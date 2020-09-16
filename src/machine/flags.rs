pub struct Flags {
  pub c: bool,
  pub ac: bool,
  pub s: bool,
  pub z: bool,
  pub p: bool,
}

impl Flags {
  pub fn new() -> Flags {
    return Flags {
      ac: false,
      c: false,
      s: false,
      z: false,
      p: false,
    }
  }

  pub fn set_flag(&mut self, result: u16) {
    self.z = if result == 0 { true } else { false};
    if result > 0xff {
      self.c = true;
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_sets_flag_z() {
    let mut flags = Flags::new();
    flags.set_flag(0);
    assert_eq!(flags.z, true);
    flags.set_flag(1);
    assert_eq!(flags.z, false);
  }
}