// use std::ops::Deref;
use std::thread;
use std::time::Duration;

fn main() {
    test();
}
struct DerefExample {}
impl Drop for DerefExample {
    fn drop(&mut self) {
        println!("drop");
    }
}
fn test() {
    println!("main start");
    for _j in 0..10 {
        thread::spawn(move || {
            test2();
        });
    }
    thread::sleep(Duration::from_secs(1));
    println!("main end2");
}

fn test2() {
    println!("DerefExample new");
    let _sky = DerefExample {};
    thread::sleep(Duration::from_secs(1))
}