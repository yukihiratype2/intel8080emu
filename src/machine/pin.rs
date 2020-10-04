use super::super::Machine;
use intel8080disassembler::Instruction;
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
    println!("{:?} ----------- interrupt ", self.pin.int);
    if self.pin.ei && self.pin.int  {
      self.pin.int = false;
      // self.registers.pc = self.pin.rst as u16;
      self.execute(&Instruction{
        opcode: 0xcd,
        operand1: 0,
        operand2: self.pin.rst
      })
    }
    return;
  }
}
