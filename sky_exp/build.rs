// extern crate cc;
// use std::{env, path::PathBuf, fs};

use std::fs::OpenOptions;
use std::env;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let mut root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let mut f1 = OpenOptions::new().truncate(true).write(true)
        .open(root.join("src").join("exp").join("git_version_file")).unwrap();
    f1.write_all(b"ttt");
    drop(f1);
}
