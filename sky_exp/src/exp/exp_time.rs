//变量

use std::time::*;
use std::thread::sleep;
// extern crate chrono;
// use chrono::{DateTime, Utc, Local};

#[allow(dead_code)]
pub fn main_time() {
    test();
}
//变量
fn test() {

    let time_interval = SystemTime::now();
    println!("main_time time interval =  {:?}",time_interval);


    sleep(Duration::new(2, 0));
    println!("main_time elapsed =  {:?} ns",time_interval.elapsed().unwrap().as_micros());
    println!("main_time elapsed =  {:?} ns",time_interval.duration_since(UNIX_EPOCH).unwrap().as_micros());
    //
    // let f_now = strftime("%Y-%m-%dT%H:%M:%S", &time_interval).unwrap();
    // println!("now: {:?}", f_now);

    // let local: DateTime<Local> = Local::now();
    // println!("now: {:?}", Local::now());
    // time_interval.duration_since(UNIX_EPOCH)

}









