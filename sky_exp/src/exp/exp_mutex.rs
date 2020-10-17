use std::{thread, time};
use std::thread::sleep;
use std::sync::{Mutex, Arc, mpsc};

#[allow(dead_code)]
pub fn main_mutex_test() {
    // test();
    // test1();
    //test2();
    test3();
}

//线程阻塞
#[allow(dead_code)]
fn test() {
    let handle = thread::spawn(|| {
        // &mut sky = String::from( "sky1111");
        println!("handle = start");
        sleep(time::Duration::from_secs(5));
        println!("handle = end");
    });

    println!("handle = {:?}", handle.join().unwrap());

    let handle1 = thread::spawn(|| {
        println!("handle1 = start");
        sleep(time::Duration::from_secs(5));
        println!("handle1 = end");
    });
    println!("handle1 = {:?}", handle1.join().unwrap());
}

//线程锁
#[allow(dead_code)]
fn test1() {
    let data=Arc::new(Mutex::new(vec![7u32, 62, 63]));

    for i in 0..3 {
        let data=data.clone();
        thread::spawn(move|| {
            let mut data=data.lock().unwrap();
            data[i] +=1;
        });
    }
    println!("data = {:?}",data);

    sleep(time::Duration::from_millis(5));
}

//通道
#[allow(dead_code)]
fn test2(){
    let data=Arc::new(Mutex::new(0u32));

    let (tx, rx) =mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move|| {
            let mut data=data.lock().unwrap();
            *data+=1;
            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    println!("data = {:?}",data);
}

//通道获取多线程数据
#[allow(dead_code)]
fn  test3(){
    let (tx, rx) =mpsc::channel();

    for i in 0..10 {
        let tx=tx.clone();

        thread::spawn(move|| {
            // let answer=42u32;

            tx.send(i).unwrap();
        });
    }
    for _ in 0..10 {
        let sky = rx.recv().ok().expect("Could not receive answer");
        println!("sky = {:?}",sky)
    }
}




