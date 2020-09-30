pub mod register;
pub mod memory;
pub mod utils;
mod pin;
mod operator;
mod interrupt;

pub struct Machine {
  pub registers: register::Registers,
  pub memory: memory::Memory,
  pub pin: pin::Pin,
  pub interrupt: interrupt::Interrupt,
}

impl Machine {
  pub fn new() -> Machine {
    let m = [0; 65535];
    return Machine{
      registers: register::Registers::new(),
      pin: pin::Pin::new(),
      memory: m,
      interrupt: interrupt::Interrupt::new(),
    }
  }

  pub fn reset(&self) -> Self {
    return Machine{
      registers: register::Registers::new(),
      pin: pin::Pin::new(),
      // TODO: memory probably not right
      memory: self.memory,
      interrupt: interrupt::Interrupt::new(),
    }
  }

  pub fn start(&mut self) {
    loop {
      self.process_cycles()
    }
  }
}