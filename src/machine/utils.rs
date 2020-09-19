use super::super::Machine;
use super::registers::{RegisterPairs};

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
        // let loc = ((self.registers.h as usize) << 8) + self.registers.l as usize;
        let loc = self.registers.get_rp(RegisterPairs::H) as usize;
        if loc > 65535 {
          panic!("Out of memory size")
        }
        return &mut self.memory[loc]
      }
      _ => {panic!("Not Implemented")}
    }
  }

  pub fn load_rom(&mut self, rom: &Vec<u8>) {
    // reset rom
    self.memory = [0; 65535];

    for (i, x) in rom.iter().enumerate() {
      self.memory[i] = *x;
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

  #[test]
  fn it_locates_memory() {
    let mut machine = Machine::new();
    machine.memory[0] = 0x01;
    machine.memory[65534] = 0x02;
    assert_eq!(*machine.locator(0b110), 0x01);
    machine.registers.set_rp(RegisterPairs::H, 0xfffe);
    assert_eq!(*machine.locator(0b110), 0x02);
  }

  #[test]
  #[ignore="not implememt"]
  fn it_throws_error() {
  }

  #[test]
  fn it_load_rom() {
    let mut machine = Machine::new();
    machine.load_rom(&vec![0xc3, 0x23, 0x22]);
    assert_eq!(machine.memory[0], 0xc3);
    assert_eq!(machine.memory[1], 0x23);
    assert_eq!(machine.memory[2], 0x22);
    machine.load_rom(&vec![0x11]);
    assert_eq!(machine.memory[0], 0x11);
    assert_eq!(machine.memory[1], 0x00);
  }
}