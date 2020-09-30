use std::fs;
use intel8080emu;

#[test]
#[ignore]
fn it_finish_test() {
    let raw = fs::read("./tests/cpudiag.bin").expect("E");
    let mut machine = intel8080emu::Machine::new();
    machine.load_rom(&raw, 0x100);
    assert_eq!(machine.memory[0x100], 0xc3);
    machine.start();
}

#[test]
// #[ignore]
fn it_run_invaders() {
    let raw = fs::read("./tests/invaders.bin").expect("E");
    let mut machine = intel8080emu::Machine::new();
    machine.load_rom(&raw, 0x0);
    assert_eq!(machine.memory[0x03], 0xc3);
    machine.start();
}