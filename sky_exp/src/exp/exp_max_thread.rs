// use std::sync::{Mutex, Arc, mpsc, RwLock};
use rayon::prelude::*;
use std::thread::JoinHandle;
use std::{thread, env};
use std::time::Duration;

#[allow(dead_code)]
pub fn main_max_thread() {
    test1();
}

/**
* 线程所有权
*
*/
fn test() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for (index, value) in args.iter().enumerate() {
        if 0 == index {
           continue
        }else{
            let num = value.trim().parse::<u64>().unwrap();
            println!("起线程数 = {:?}", num);
            let _sky = vec![1; num as usize].into_par_iter()
                .map(|_prover| {
                    let mut i = 0;
                    for _index in 0..1000 * 1000 {
                        i = 1 + 1;
                        thread::sleep(Duration::from_millis(10));
                    }
                }).collect::<Vec<_>>();
        }
    }
}



fn test1() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for (index, value) in args.iter().enumerate() {
        if 0 == index {
            continue
        }else{
            let num = value.trim().parse::<u64>().unwrap();
            println!("起线程数 = {:?}", num);
            let mut  list:Vec<JoinHandle<()>>  = vec![];
            for _i in 0..num {
                let sky = thread::spawn( move|| {
                    let mut i = 0;
                    for _index in 0..1000 * 1000 {
                        i = 1 + 1;
                        thread::sleep(Duration::from_millis(10));
                    }
                });
                list.push(sky);
            }
            for item in list {
                item.join().unwrap()
            }

        }
    }
}