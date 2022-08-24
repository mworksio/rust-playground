use std::{thread, thread::JoinHandle, time::Duration, sync::mpsc};

fn main() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val: i32 = receiver.recv().unwrap();
        val + 5
    });

    sender.send(8).unwrap();
    println!("result = {}", handle.join().unwrap());
}
