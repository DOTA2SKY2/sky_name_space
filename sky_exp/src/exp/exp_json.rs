use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::path::Path;

pub fn main_json() {
    test3();
    // let file = value_t!(m, "config", String).unwrap();
}

// #[derive(Deserialize, Debug)]
// struct User {
//     fingerprint: String,
//     location: String,
// }
//
// fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<Error>> {
//     // Open the file in read-only mode with buffer.
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//
//     // Read the JSON contents of the file as an instance of `User`.
//     let u = serde_json::from_reader(reader)?;
//
//     // Return the `User`.
//     Ok(u)
// }
//
// fn test() {
//     let u = read_user_from_file("/gpu.json").unwrap();
//     println!("{:#?}", u);
// }


#[derive(Deserialize, Debug)]
struct GpuCfg {
    is_seal_model: bool,
    verify_cnt: usize,
    winning_post_cnt: usize,
    p2_cnt: usize,
    c2_cnt: usize,
}
// #[derive(Deserialize, Debug)]
// enum  GpuTypeConfig {
//     Verify(i32),
//     WinningPost(i32),
//     P2(i32),
//     C2(i32),
// }

fn test2() {
    // The type of `j` is `&str`
    let j = "
        {
            \"is_seal_model\" : true,
            \"verify_cnt\": 1,
            \"winning_post_cnt\": 1,
            \"p2_cnt\": 2,
            \"c2_cnt\": 2
         }";

    let u: GpuCfg = serde_json::from_str(j).unwrap();

    println!("{:#?}", u);
}


fn test3(){

   // let file =  File::open("gpu.cfg");
    // match file {
    //     Ok(x) => {
    //
    //     }
    //     Err(w) => {
    //
    //     }
    // }
    let path = Path::new("gpu.cfg");
    match File::open(&path) {
        Err(why) => {
            dbg!(why);
            let mut file =  File::create(&path).unwrap();
            let j = "
        {
            \"is_seal_model\" : true,
            \"verify_cnt\": 1,
            \"winning_post_cnt\": 1,
            \"p2_cnt\": 2,
            \"c2_cnt\": 2
         }";
            file.write_all(j.as_bytes());
        }
        Ok(file) => {

        },
    };
    let file = File::open(&path).unwrap();

    // dbg!(path)
    let reader = BufReader::new(file);
    let u:GpuCfg = serde_json::from_reader(reader).unwrap();
    dbg!(u);
}