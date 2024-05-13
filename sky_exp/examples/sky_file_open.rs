// use tempfile::{NamedTempFile, Builder};
// use std::io::{SeekFrom, Seek, Read,Write};
// use std::env;
// use std::fs::{File, OpenOptions};
// use std::path::Path;

use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

#[allow(dead_code)]

fn main() {
    test_dir();
}

#[allow(dead_code)]
fn test_basic() {
  let mut  file  =  OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./stacked-data.txt")
        .unwrap();
    // let piece_bytes: Vec<u8> = (0..2) //切片
    //     .map(|_| 11)
    //     .collect();

    let piece_bytes = vec![1, 2, 3, 4, 5];
    &file.set_len(32 * 1 as u64).unwrap();
   &file.write(&piece_bytes);
}
const PATH_PREFIX: &str = "/Users/sky/rust_test_exp/test1";
fn test_dir() {
    fs::remove_dir_all(PATH_PREFIX);
    fs::create_dir_all(PATH_PREFIX); // 没有父目录的时候，顺便生成父目录
    fs::create_dir(PATH_PREFIX); // 没有父目录的时候，报错

}

