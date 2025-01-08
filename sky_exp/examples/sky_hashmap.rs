// use std::ops::{AddAssign, MulAssign};
use std::sync::Mutex;
use std::collections::HashMap;
// use syn::Error;


const GPU_VERIFY_CNT: i32 = 1;
const GPU_WINNING_POST_CNT: i32 = 1;
const GPU_P2CNT: i32 = 2;
const GPU_C2CNT: i32 = 4;
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
fn init_device_pools() -> Mutex<HashMap<i32, (GpuType, i32)>> {
    let device_pools: Mutex<HashMap<i32, (GpuType, i32)>> = Mutex::new(HashMap::new());
    let mut gpu_type = GpuType::P2;
    for i in 0..6 {
        if i < GPU_P2CNT {
            gpu_type = GpuType::P2;
        } else {
            gpu_type = GpuType::C2;
        }
        device_pools.lock().unwrap().insert(i, (gpu_type, 0));
    }
    device_pools
}

#[allow(dead_code)]
fn main() {
    test_1();
}


fn test_1() {
    pub fn get_device(gpu_type: GpuType) -> Result<i32, &'static str> {
        for (k, v) in GET_DEVICE_POOLS.lock().unwrap().iter_mut() {
            if v.0 as i32 == gpu_type as i32 {
                if v.1 == 0 {
                    v.1 = 1;
                    return Ok(*k);
                }
            }
        }
        Err("wwww")
    }

    let item = get_device(GpuType::P2).unwrap();
    println!("{:?}", item);

    for (k, v) in GET_DEVICE_POOLS.lock().unwrap().iter_mut() {
        println!("v.0 ={:?}", v.0);
        println!("v.1 ={:?}", v.1);
        println!("k ={:?}", k);
    }
}


fn test_hash_map(){
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        let sky = map.get(&key).is_some();
        if sky {
            map.get(&key).unwrap()
        } else {
            map.insert(key.clone(), V::default());
            map.get(&key).unwrap()
        }
    }
}

