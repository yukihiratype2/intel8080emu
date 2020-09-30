use super::super::Machine;

pub struct Interrupt {}

impl Interrupt {
  pub fn new() -> Self {
    Interrupt {}
  }
}

impl Machine{
  pub fn check_interrupt(&self) {
    if self.pin.int {}
    return
  }
}