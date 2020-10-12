use std::fs;
use intel8080emu;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use shift_register::ShiftRegister;


#[test]
// #[ignore]
fn it_run_invaders() {
    let raw = fs::read("./tests/invaders.bin").expect("E");
    let sr = Arc::new(Mutex::new(ShiftRegister::new()));

    let f = & |x:u8| {
        sr.clone().lock().unwrap().write(x);
        x
    };
    let machine = Arc::new(Mutex::new(intel8080emu::Machine::new(Some([
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
        Box::new(f.clone()),
    ]))));

    let (tx, rx) = mpsc::channel();


    machine.lock().unwrap().load_rom(&raw, 0x0);

    thread::spawn(move || {
        loop {
            tx.send(true).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    let interrupt = machine.clone().lock().unwrap().pin.int.clone();
    let rst = machine.clone().lock().unwrap().pin.rst.clone();
    std::thread::spawn(move || {
        for _received in rx {
            // m = true;
            *interrupt.lock().unwrap() = true;
            *rst.lock().unwrap() = 0x10;
            // m.lock().unwrap().pin.rst = 0x10;
            // m.lock().unwrap().pin.int = true;
        }
    });
    let m2 = machine.clone();
    loop {
        m2.lock().unwrap().process_cycles();
    }

}