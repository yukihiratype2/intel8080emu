pub mod register;
pub mod memory;
pub mod utils;
mod pin;
mod operator;
mod interrupt;

pub struct Machine<'a> {
  pub registers: register::Registers,
  pub memory: memory::Memory,
  pub pin: pin::Pin<'a>,
  pub interrupt: interrupt::Interrupt,
}

impl<'a> Machine<'a> {
  pub fn new(ports: pin::Ports) -> Machine {
    let m = [0; 65535];
    return Machine{
      registers: register::Registers::new(),
      pin: pin::Pin::new(ports),
      memory: m,
      interrupt: interrupt::Interrupt::new(),
    }
  }

  // pub fn reset(&self) -> Self {
  //   return Machine{
  //     registers: register::Registers::new(),
  //     pin: pin::Pin::new(),
  //     // TODO: memory probably not right
  //     memory: self.memory,
  //     interrupt: interrupt::Interrupt::new(),
  //   }
  // }

  pub fn start(&mut self) {
    loop {
      self.process_cycles()
    }
  }
}
