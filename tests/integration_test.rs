use intel8080emu;

#[test]
fn it_adds_two() {
    let machine = intel8080emu::Machine::new();
    assert_eq!(false, machine.flags.z);
}