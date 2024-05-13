use std::thread;
use std::time::Duration;

//变量
#[allow(dead_code)]




fn main() {
    for _ in 0..10 {
        thread::spawn(move || {
            test2() ;
        });
    }
    thread::sleep(Duration::from_secs(100))
}

fn test2() {
    let sky = GpuProcessLock::new();
    panic!("panic");
    // drop(sky);
}


struct GpuProcessLock {
    cnt: i32
}

impl GpuProcessLock {
    pub fn new() -> GpuProcessLock {
        println!("new");
        GpuProcessLock {
            cnt: 0
        }
    }
}

impl Drop for GpuProcessLock
{
    fn drop(&mut self) {
        println!("drop");
    }
}

