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

pub fn main_arc() {

    let mut cs = A { foo: 8 };
    let a = Arc::new(Mutex::new(cs));

    let mut map: HashMap<u8, Arc<Mutex<dyn Foo>>> = HashMap::new();
    map.insert(8u8, a);

    for _ in 0..2 {
        let a = map.get(&8u8).expect("boom").clone();
        thread::spawn(move || {
            let result = a.lock().ok().expect("boom indeed").get_foo();
            println!("Result: {}", result);
        });
    }
    let a1 = map.get(&8u8).expect("boom").clone();
    let mut data = a1.lock().unwrap();
     let data1 = *data;
    let re = data1.get_foo();
    println!("{:?}",re);
    thread::sleep(Duration::from_millis(200));
}