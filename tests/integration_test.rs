use std::fs;
use intel8080emu;

#[test]
// #[ignore]
fn it_finish_test() {
    let raw = fs::read("./tests/cpudiag.bin").expect("E");
    let mut machine = intel8080emu::Machine::new(vec!());
    machine.load_rom(&raw, 0x100);
    assert_eq!(machine.memory[0x100], 0xc3);
    machine.start();
}

#[test]
// #[ignore]
fn it_run_invaders() {
    let raw = fs::read("./tests/invaders.bin").expect("E");
    let mut machine = intel8080emu::Machine::new(vec!(
        &|x| {println!("0: {:?}", x);x},
        &|x| {println!("1: {:?}", x);x},
        &|x| {println!("2: {:?}", x);x},
        &|x| {println!("3: {:?}", x);x},
        &|x| {println!("4: {:?}", x);x},
        &|x| {println!("5: {:?}", x);x},
        &|x| {println!("6: {:?}", x);x},
        &|x| {println!("7: {:?}", x);x},
        &|x| {println!("8: {:?}", x);x},
    ));
    machine.load_rom(&raw, 0x0);
    assert_eq!(machine.memory[0x03], 0xc3);
    machine.start();
}