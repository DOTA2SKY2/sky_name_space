// use serde::Deserialize;
// use std::sync::{Mutex, Arc};
use std::{process, thread, fs};
// use std::fs::File;
// use std::path::PathBuf;
// use std::io::Read;
// use std::sync::atomic::Ordering;

use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{Write, Read, Seek, BufReader, SeekFrom};
use std::time::Duration;
use std::path::PathBuf;
// use std::fs::{File, OpenOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::Any;

// use fs2::FileExt;
// use std::fs::File;
use std::fs::*;
use fs2::*;
use super::*;
// extern crate tempdir;
use tempdir::TempDir;
// use std::io::{Read, Seek, SeekFrom, Write};


#[derive(Clone, Debug, Copy)]
pub enum GpuType {
    Verify,
    WinningPost,
    P2,
    C2,
}

#[derive(Deserialize, Serialize, Debug)]
struct GpuList {
    // #[serde(rename(serialize = "persons", deserialize = "persons"))]
    pub gpu_list: HashMap<i32, GpuInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GpuInfo {
    pub is_used: bool,
    pub process_id: usize,
    pub thread_id: i32,

}

pub fn main_file_lock_json() {
    // test1()
    // test2()
    test5()
}

lazy_static::lazy_static! {
    pub static ref THREAD_CNT: Mutex<i32> = Mutex::new(0);
}
const GPU_LIST_CFG: &str = "gpu_list.cfg";

fn tmp_path(filename: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(filename);
    p
}


fn test() {
    // let file1 = File::create(tmp_path(GPU_LIST_CFG)).unwrap();
    // let reader = BufReader::new(file1);
    // let cfg: GpuCfg = serde_json::from_reader(reader).unwrap();
    // println!("cfg = {:?}",cfg);
    // let mut file = File::create(tmp_path(GPU_LIST_CFG)).unwrap();

    let path = tmp_path(GPU_LIST_CFG);
    println!("path = {:?}", path);
    if !path.exists() {
        let mut list: GpuList = GpuList {
            gpu_list: HashMap::new()
        };
        list.gpu_list.insert(0, GpuInfo {
            is_used: false,
            process_id: 0,
            thread_id: 0,
        });
        list.gpu_list.insert(1, GpuInfo {
            is_used: false,
            process_id: 0,
            thread_id: 0,
        });
        list.gpu_list.insert(2, GpuInfo {
            is_used: false,
            process_id: 0,
            thread_id: 0,
        });
        serde_json::to_writer(&mut File::create(&path).unwrap(), &list).unwrap();
    }
    let str = fs::read_to_string(&path).unwrap();
    let mut cfg: GpuList = serde_json::from_str(&str).unwrap();
    println!("file init = {:?}", cfg);
    // let sky = thread::current().id() as u64;
    // letsky1:u64 =  sky.;
    cfg.gpu_list.iter_mut().for_each(|(key, value)| {
        value.process_id = 100;
        // value.thread_id  = thread::current().id().to_string();
    });

    let sky = cfg.gpu_list.get_mut(&0).unwrap();
    sky.thread_id = 99999;
    // cfg.gpu_list.remove(0);
    serde_json::to_writer(&mut File::create(&path).unwrap(), &cfg).unwrap();
    let mut cfg: GpuList = serde_json::from_reader(File::open(&path).unwrap()).unwrap();
    println!("cfgyyyy = {:?}", cfg);


    // File::create(GPU_LIST_CFG).
    // file.write_all(json_str.as_bytes());
    // // drop(file);
    // file.seek(SeekFrom::Start(0));
    //
    // let mut buffer = String::new();
    // file.seek(SeekFrom::Start(0));
    // file.read_to_string(&mut buffer);
    // println!("buffer = {:?}",buffer);
    // let mut buffer1 = String::new();
    // file.seek(SeekFrom::Start(0));
    // file.read_to_string(&mut buffer1);
    // println!("buffer1 = {:?}",buffer1);
    // // let file1 = File::open(file).unwrap();
    // // file1.seek(SeekFrom::Current(0));
    // file.seek(SeekFrom::Start(0));
    // let reader = BufReader::new(&file);
    // let mut cfg: GpuInfo = serde_json::from_reader(reader).unwrap();
    // println!("cfgyyyy = {:?}",cfg);
    //
    // cfg.c2_cnt = 1000;
    // file.seek(SeekFrom::Start(0));
    // serde_json::to_writer(&mut file, &cfg).unwrap();
    // println!("cfgwwwww = {:?}",cfg);
    //
    // let mut buffer = String::new();
    // file.seek(SeekFrom::Start(0));
    // file.read_to_string(&mut buffer);
    // println!("buffer = {:?}",buffer);


    // file.seek(SeekFrom::Start(0));
    // let reader = BufReader::new(&file);
    // let mut cfg: GpuCfg = serde_json::from_reader(reader).unwrap();
    // println!("hhhhhh = {:?}",cfg);
}


fn test2() {
    duplicate()
}

fn duplicate() {
    let tempdir = TempDir::new("fs2").unwrap();
    let path = tempdir.path().join("fs2");
    let mut file1 =
        fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

    file1.lock_exclusive();
    let mut file2 = file1.duplicate().unwrap();
// file1.truncate()
    // Write into the first file and then drop it.
    file1.write_all(b"foo").unwrap();
    drop(file1);


    let mut buf = vec![];

    // Read from the second file; since the position is shared it will already be at EOF.
    file2.read_to_end(&mut buf).unwrap();
    println!("buf = {:?}", buf);
    assert_eq!(0, buf.len());

    // Rewind and read.
    file2.seek(SeekFrom::Start(0)).unwrap();
    file2.read_to_end(&mut buf).unwrap();
    println!("buf = {:?}", buf);
    assert_eq!(&buf, &b"foo");
}


fn test5() {

    for i in 0..2 {
        thread::spawn(move || {
            let path = tmp_path("1.lock");
            println!("path = {:?}", path);
            let file1 = File::create(path).unwrap();

            println!("thread  lock_exclusive start");
            let sky = file1.lock_exclusive();
            match sky {
                Ok(_0) => {
                    println!(" ok")
                }
                Err(_0) => {
                    println!("Err")
                }
            }
            println!("thread = lock_exclusive ing");
            thread::sleep(Duration::from_secs(10));
            file1.unlock();
            println!("thread = lock_exclusive end");
        });
    }

    thread::sleep(Duration::from_secs(2));

    for i in 0..1000 {
        let path = tmp_path("1.lock");
        let mut file1 = File::create(path).unwrap();
        let sky = file1.try_lock_exclusive();
        match sky {
            Ok(_0) => {
                println!("try_lock_exclusive ok")
            }
            Err(_0) => {
                println!("try_lock_exclusive Err")
            }
        }
        thread::sleep(Duration::from_secs(1))
    }
    // file1.lock_exclusive();
    // // file1.lock_exclusive();
    // file1.unlock();
    // let sky = file1.try_lock_exclusive();
    // match sky {
    //     Ok(_0) => {
    //         println!("ok")
    //     }
    //     Err(_0) => {
    //         println!("Err")
    //     }
    // }

    thread::sleep(Duration::from_secs(400))
}
