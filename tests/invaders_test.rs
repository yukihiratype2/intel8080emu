use std::fs;
use intel8080emu;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;


#[test]
#[ignore]
fn it_run_invaders() {
    let raw = fs::read("./tests/invaders.bin").expect("E");
    let machine = Arc::new(Mutex::new(intel8080emu::Machine::new(vec!(
        &|x| {println!("0: {:?}", x);x},
        &|x| {println!("1: {:?}", x);x},
        &|x| {println!("2: {:?}", x);x},
        &|x| {println!("3: {:?}", x);x},
        &|x| {println!("4: {:?}", x);x},
        &|x| {println!("5: {:?}", x);x},
        &|x| {println!("6: {:?}", x);x},
        &|x| {println!("7: {:?}", x);x},
        &|x| {println!("8: {:?}", x);x},
    ))));

    let (tx, rx) = mpsc::channel();


    machine.lock().unwrap().load_rom(&raw, 0x0);

    thread::spawn(move || {
        loop {
            tx.send(true).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    let m = machine.clone();
    std::thread::spawn(move || {
        for _received in rx {
            m.lock().unwrap().pin.rst = 0x10;
            m.lock().unwrap().pin.int = true;
        }

    });
    let m2 = machine.clone();
    loop {
        m2.lock().unwrap().process_cycles();
    }

}