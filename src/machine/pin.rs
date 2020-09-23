// use super::super::Machine;

pub struct Pin {
  pub a0: bool,
  pub a1: bool,
  pub a2: bool,
}

impl Pin {
  pub fn new() -> Self {
    return Pin {
      a0: false,
      a1: false,
      a2: false,
    }
  }
}