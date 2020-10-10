use super::super::Machine;
pub type Ports<'a> = Vec<&'a (dyn Fn(u8) -> u8 + Sync)>;

pub struct Pin<'a> {
  pub int: bool,
  pub ports: Ports<'a>,
  pub rst: u8,
  pub ei: bool
}

impl<'a> Pin<'a> {
  pub fn new(ports: Ports) -> Pin {
    return Pin {
      int: false,
      ports: ports,
      rst: 0,
      ei: false
    };
  }
}

impl<'a> Machine<'a> {
  pub fn check_interrupt(&mut self) {
    if self.pin.ei && self.pin.int  {
      self.pin.int = false;
      // self.registers.pc = self.pin.rst as u16;
        self.memory[self.registers.sp as usize - 1] = (((self.registers.pc) & 0xff00) >> 8) as u8;
        self.memory[self.registers.sp as usize - 2] = ((self.registers.pc) & 0xff) as u8;
        self.registers.sp -= 2;
        self.registers.pc = self.pin.rst as u16;
    }
    return;
  }
}
