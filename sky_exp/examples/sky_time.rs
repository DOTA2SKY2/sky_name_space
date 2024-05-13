//变量

// use std::time::*;

use log::info;
use chrono::NaiveTime;
extern crate time as NewTime;

use NewTime::Duration as NewDuration;
// use chrono::NaiveTime;
// extern crate chrono;
// use chrono::{DateTime, Utc, Local};


#[allow(dead_code)]
fn main() {
    test1();
}
//变量
fn test() {

    // let time_interval = SystemTime::now();
    // println!("main_time time interval =  {:?}",time_interval);
    //
    //
    // sleep(Duration::new(2, 0));
    // println!("main_time elapsed =  {:?} ns",time_interval.elapsed().unwrap().as_micros());
    // println!("main_time elapsed =  {:?} ns",time_interval.duration_since(UNIX_EPOCH).unwrap().as_micros());
    //
    // let f_now = strftime("%Y-%m-%dT%H:%M:%S", &time_interval).unwrap();
    // println!("now: {:?}", f_now);

    // let local: DateTime<Local> = Local::now();
    // println!("now: {:?}", Local::now());
    // time_interval.duration_since(UNIX_EPOCH)

}

fn test1() {
    let from_hmsm = NaiveTime::from_hms_milli;
    let time1 = from_hmsm(0, 0, 0, 0) + NewDuration::seconds(10000000);

    let str = time1.format("%H:%M:%S").to_string();
    info!("from_hmsm =  {:?} ", str);


}










