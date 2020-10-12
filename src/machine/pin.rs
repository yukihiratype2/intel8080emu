use super::super::Machine;
use std::sync::{Arc, Mutex};
pub type Ports<'a> = Option<[Box<dyn FnMut(u8) -> u8 + 'a>; 8]>;

pub struct Pin<'a> {
  pub int: Arc<Mutex<bool>>,
  pub ports: Ports<'a>,
  pub rst: Arc<Mutex<u8>>,
  pub ei: bool
}

impl<'a> Pin<'a> {
  pub fn new(ports: Ports) -> Pin {
    return Pin {
      int: Arc::new(Mutex::new(false)),
      ports: ports,
      rst: Arc::new(Mutex::new(0)),
      ei: false
    };
  }
}

impl<'a> Machine<'a> {
  pub fn check_interrupt(&mut self) {
    if self.pin.ei && *self.pin.int.clone().lock().unwrap() {
      // self.pin.int = false;
      *self.pin.int.lock().unwrap() = false;
      // self.registers.pc = self.pin.rst as u16;
        self.memory[self.registers.sp as usize - 1] = (((self.registers.pc) & 0xff00) >> 8) as u8;
        self.memory[self.registers.sp as usize - 2] = ((self.registers.pc) & 0xff) as u8;
        self.registers.sp -= 2;
        // self.registers.pc = self.pin.rst as u16;
        self.registers.pc = *self.pin.rst.clone().lock().unwrap() as u16;
    }
    return;
  }
}
