use once_cell::sync::OnceCell;
use std::{thread, process};
use std::time::Duration;
use std::thread::JoinHandle;

pub fn main_once_cell() {
    // for i in 1..10 {
    //   println!("www = {:?}", test(i))  ;
    // }
    let mut  list:Vec<JoinHandle<()>>  = vec![];
    for i in 0..10 {
        let sky = thread::spawn(move || {
            println!("111 = {:?}",test(i))  ;
        });
        // thread::sleep(Duration::from_secs(1 ));
        list.push(sky);
    }

    for item in list {
        item.join().unwrap()
    }

}

fn test(index :i32) ->  &'static i32 {
    static INSTANCE_32_GIB: OnceCell<i32> = OnceCell::new();
    INSTANCE_32_GIB.get_or_init(|| {
        println!("wwwww");
        index
    })
}