use std::{thread, thread::JoinHandle, time::Duration};
use rand::{self, Rng};

fn main() {
    /*
    let handle: JoinHandle<()> = thread::spawn( || {
        let delay = rand::thread_rng().gen_range(10..=2000);
        thread::sleep(Duration::from_millis(delay));
        println!("Hello from spawned thread");
    });
    handle.join();
    */

    /*
    let handles: Vec<JoinHandle<String>> = (0..=10).map( |i| {
        let delay = rand::thread_rng().gen_range(10..=2000);
        let builder = thread::Builder::new().name(format!("Thread-{}", i));
        
        builder.spawn(move || {
            println!("thread started = {}", thread::current().name().unwrap());
            thread::sleep(Duration::from_millis(delay));
            thread::current().name().unwrap().to_owned()
        }).unwrap();
    }).collect();

    for h in handles {
        let r = h.join().unwrap();
        println!("thread done = {:?}", r);
    }
    */
/*
    let handles: Vec<JoinHandle<String>> = (0..=10).map(|i| {
  let delay = rand::thread_rng().gen_range(10..=2000);
  let builder =
    thread::Builder::new().name(format!("Thread-{}", i));

  builder.spawn(move || {
    println!(
      "thread started = {}",
      thread::current().name().unwrap()
    );
    thread::sleep(Duration::from_millis(delay));
    thread::current().name().unwrap().to_owned()
  }).unwrap();
}).collect();
for h in handles {
  let r = h.join().unwrap();
  println!("thread done = {:?}", r);
}
*/
}
