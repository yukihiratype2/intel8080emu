use intel8080emu;

#[test]
fn it_adds_two() {
    let machine = intel8080emu::Machine::new();
    assert_eq!(0, machine.flags.z());
}