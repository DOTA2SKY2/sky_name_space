// extern crate cc;
// use std::{env, path::PathBuf, fs};

use std::fs::OpenOptions;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use vergen::{ConstantsFlags, generate_cargo_keys};
fn main() {
    // let mut root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    // let mut f1 = OpenOptions::new().truncate(true).write(true)
    //     .open(root.join("src").join("exp").join("git_version_file")).unwrap();
    // f1.write_all(b"ttt");
    // drop(f1);


    // let mut flags = ConstantsFlags::all();
    // flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
    // flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
    // flags.toggle(ConstantsFlags::BUILD_DATE);
    // flags.toggle(ConstantsFlags::SHA);
    // flags.toggle(ConstantsFlags::SHA_SHORT);
    // flags.toggle(ConstantsFlags::COMMIT_DATE);
    // flags.toggle(ConstantsFlags::TARGET_TRIPLE);
    // flags.toggle(ConstantsFlags::SEMVER);
    // flags.toggle(ConstantsFlags::SEMVER_LIGHTWEIGHT);

    // Generate the 'cargo:' key output
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");
}