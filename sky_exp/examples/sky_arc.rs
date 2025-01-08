use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
struct A {
    foo: u8,
}

trait Foo: Send {
    fn get_foo(&self) -> u8;
}

impl Foo for A {
    fn get_foo(&self) -> u8 {
        self.foo
    }
}

fn main() {
    let cs: A = A { foo: 8 };
    let a = Arc::new(Mutex::new(cs));

    let mut map: HashMap<u8, Arc<Mutex<dyn Foo>>> = HashMap::new();
    map.insert(8u8, a);

    for _ in 0..3 {
        let a = map.get(&8u8).expect("boom").clone();
        thread::spawn(move || {
            let tid =  thread::current().id();
            println!("thread::{:?}",tid);

            let result = a.lock().ok().expect("boom indeed").get_foo();
            println!("thread::{:?}; Result: {}",  tid, result);
        });
    }

    let a1 = map.get(&8u8).expect("boom").clone();
    println!("main thread::{:?}, Result:{:?}", thread::current().id(), a1.lock().ok().expect("main thread").get_foo());
    thread::sleep(Duration::from_secs(2));
}