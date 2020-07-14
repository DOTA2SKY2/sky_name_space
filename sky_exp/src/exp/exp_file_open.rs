// use tempfile::{NamedTempFile, Builder};
// use std::io::{SeekFrom, Seek, Read,Write};
// use std::env;
// use std::fs::{File, OpenOptions};
// use std::path::Path;

use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]

pub fn main_file_open() {
    test_basic();
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
