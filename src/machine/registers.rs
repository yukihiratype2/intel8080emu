#[derive(Debug)]
pub struct Registers {
  pub a: u8,
  pub b: u8,
  // c: u8,
  // d: u8,
  // e: u8,
  pub h: u8,
  pub l: u8,
}

impl Registers {
  pub fn new() -> Registers {
    return Registers {
      a: 0,
      b: 0,
      h: 0,
      l: 0,
    }
  }
}