// use std::ops::{AddAssign, MulAssign};
use std::sync::Mutex;
use std::collections::HashMap;
// use syn::Error;

#[allow(dead_code)]
pub fn main_hashmap() {
    test_1();
}
const GpuVerifyCnt: i32 = 1;
const GpuWinningPostCnt: i32 = 1;
const GpuP2Cnt: i32 = 2;
const GpuC2Cnt: i32 = 4;
// pub struct Device{
//     id:i32
// }
#[derive(Clone, Debug, Copy)]
pub enum GpuType {
    Verify,
    WinningPost,
    P2,
    C2,
}

lazy_static::lazy_static! {
    pub static ref GET_DEVICE_POOLS: Mutex<HashMap<i32, (GpuType ,i32)>> = init_device_pools();
}
#[allow(dead_code)]
fn init_device_pools() -> Mutex<HashMap<i32, (GpuType ,i32)>>{
   let device_pools: Mutex<HashMap<i32, (GpuType ,i32)>> = Mutex::new(HashMap::new()) ;
    let mut gpuType = GpuType::P2;
    for i in 0..6 {

        if i < GpuP2Cnt {
            gpuType = GpuType::P2;
            // device_pools.lock().unwrap().insert(*value, (GpuType::P2, 0));
        } else {
            gpuType = GpuType::C2;
        }
        // let device = Device{
        //     id :1,
        // };

        device_pools.lock().unwrap().insert(i, (gpuType, 0));
        // println!("GPU BUS_ID  = {:?}, GpuType = {:?}",value.info_raw(0x4008), gpuType);
    }
    device_pools
}

fn test_1(){
   let item = get_device(GpuType::P2).unwrap();
    println!("{:?}",item);

    for (k, v) in GET_DEVICE_POOLS.lock().unwrap().iter_mut() {
        println!("v.0 ={:?}",v.0);
        println!("v.1 ={:?}",v.1);
        println!("k ={:?}",k);
    }
}

pub fn get_device(gpuType: GpuType) -> Result<i32,&'static str> {
    for (k, v) in GET_DEVICE_POOLS.lock().unwrap().iter_mut() {
        // println!("v.0 ={:?}",v.0);
        // println!("v.1 ={:?}",v.1);
        // println!("k ={:?}",k);
        if v.0 as i32 == gpuType as i32 {
            if v.1 == 0 {
                v.1 = 1;
                // println!("uuuu v.0 ={:?}",v.0);
                // println!("uuuu v.1 ={:?}",v.1);
                // println!("uuuu k ={:?}",k);
                return Ok(*k);
            }
        }
    }
    Err("wwww")
}