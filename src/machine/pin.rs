// use super::super::Machine;
pub type Ports<'a> = Vec<&'a dyn Fn(u8) -> u8>;

pub struct Pin<'a> {
  pub int: bool,
  pub ports: Ports<'a>
}

impl<'a> Pin<'a> {
  pub fn new(ports: Ports) -> Pin {
    return Pin {
      int: false,
      ports: ports
    }
  }
}
