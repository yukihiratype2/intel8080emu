use super::super::Machine;

impl Machine {
  pub fn locator(&mut self, reg: u8) -> &mut u8 {
    match reg {
      0b111 => {
        return &mut self.registers.a
      }
      0b000 => {
        return &mut self.registers.b
      }
      0b110 => {
        let loc = (self.registers.h as usize) << 8 + self.registers.l as usize;
        return &mut self.memory[loc]
      }
      _ => {panic!("Not Implemented")}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_locates_register() {
    let mut machine = Machine::new();
    machine.registers.b = 0x01;
    assert_eq!(*machine.locator(0), 0x01);
  }
}