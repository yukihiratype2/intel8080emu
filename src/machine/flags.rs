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
    self.c = if result > 0xff { true } else { false};
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
    flags.set_flag(0);
    assert_eq!(flags.z, true);
  }

  #[test]
  //TODO: confirm flag works correctly
  fn it_sets_flag_c() {
    let mut flags = Flags::new();
    flags.set_flag(0);
    assert_eq!(flags.c, false);
    flags.set_flag(0xff);
    assert_eq!(flags.c, false);
    flags.set_flag(0xff + 1);
    assert_eq!(flags.c, true);
    flags.set_flag(0);
    assert_eq!(flags.c, false);
  }
}