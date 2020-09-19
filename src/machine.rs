pub mod registers;
pub mod memory;
mod utils;
mod operator;

pub struct Machine {
  pub registers: registers::Registers,
  pub memory: memory::Memory,
}

impl Machine {
  pub fn new() -> Machine {
    let m = [0; 65535];
    return Machine{
      registers: registers::Registers::new(),
      memory: m
    }
  }
}