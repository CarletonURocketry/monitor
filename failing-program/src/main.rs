use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(1));
    panic!("Crashing now!");
}
