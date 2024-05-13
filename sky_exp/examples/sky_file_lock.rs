// use serde::Deserialize;
// use std::sync::{Mutex, Arc};
use std::{thread};
// use std::fs::File;
// use std::path::PathBuf;
// use std::io::Read;
// use std::sync::atomic::Ordering;

use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
// use std::io::{Write, Seek};
// use std::time::Duration;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};

fn main() {
    // dbg!(process::id());
    // dbg!(thread::current().id());
    // test()
    test2()
}

lazy_static::lazy_static! {
    pub static ref THREAD_CNT: Mutex<i32> = Mutex::new(0);
}

fn test() {
    for _i in 0..8 {
        let mut jj = THREAD_CNT.lock().unwrap();
        *jj += 1;
        if 6 == *jj {
            dbg!("lll");
        }
        // dbg!(jj);
    }
}

fn test1() {
    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let share_var = var.clone();

    // 创建一个新线程
    let new_thread = thread::spawn(move || {
        let ww = share_var.load(Ordering::SeqCst);
        let ww = ww + 1;

        println!("sky =  {:?}", ww == 6);
        println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));

        // 修改值
        share_var.store(7, Ordering::SeqCst);
    });

    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}


fn test2() {
    // for i in 0..2 {
    //     let new_thread = thread::spawn(move|| {
    //         let lock = GPULock::lock();
    //         thread::sleep(Duration::from_secs(2));
    //         drop(lock)
    //     });
    // }
    // thread::sleep(Duration::from_secs(20));
    let _lock = GPULock::lock();
    // let mut f = File::create(tmp_path(GPU_LOCK_NAME)).unwrap();
    let _buffer = String::new();
    // f.read_to_string(&mut buffer);
    // println!("1111111 ={:?}", buffer);

    // lock.read_to_string(&mut buffer);
    // println!("ggg ={:?}", buffer);
    // let lock2 = GPULock::lock();

    // drop(lock)
}

pub struct GPULock(File);

impl GPULock {
    pub fn lock() -> GPULock {
        // debug!("Acquiring GPU lock...");
        // let mut f = File::create(tmp_path(GPU_LOCK_NAME)).unwrap();


        let f1 = OpenOptions::new().read(true)
            .open(tmp_path(GPU_LOCK_NAME)).unwrap();
        // let mut buffer = String::new();
        // f1.read_to_string(&mut buffer);
        // drop(f1);
        //
        // if buffer.is_empty(){
        //     //加锁
        //     println!("pid == process::id()");
        //     let mut f = OpenOptions::new().truncate(true).write(true).create(true)
        //         .open(tmp_path(GPU_LOCK_NAME)).unwrap();
        //     f.write_all( process::id().to_string().as_bytes());
        //     drop(f);
        // }else if buffer == process::id().to_string() {
        //     //枷锁
        // }else{
        //
        // }
        // if buffer == process::id().to_string() {
        //
        // }else{
        //
        // }
        // println!("55555 ={:?}", pid);
        // let mut buffer = String::new();
        // f.read_to_string(&mut buffer);
        // // process::id().to_string()
        // if buffer ==  process::id().to_string() {
        //     println!("read process::id() = {:?}",process::id());
        // }else {
        //     f.write_all(process::id().as_bytes());
        //     println!("write_all process::id() = {:?}",process::id());
        // }
        // f.lock_exclusive().unwrap();
        // debug!("GPU lock acquired!");
        GPULock(f1)
    }
}

impl Drop for GPULock {
    fn drop(&mut self) {
        // debug!("GPU lock released!");
    }
}

const GPU_LOCK_NAME: &str = "bellman.gpu.lock";
const PRIORITY_LOCK_NAME: &str = "bellman.priority.lock";

fn tmp_path(filename: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    println!("tmp {:?}", p);
    p.push(filename);
    p
}