use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, mpsc};
use std::sync::atomic::Ordering::Relaxed;
use std::thread::current;
use tokio::{runtime, task};
use tokio::runtime::Runtime;

/// 在异步编程下，多个任务执行，用 原子计数器递减+channel 在判断所有任务是否完成
fn main() {
    const NUM_SPAWN: usize = 10_000;

    let rt = rt();

    let (tx, rx) = mpsc::sync_channel(1000);
    let rem = Arc::new(AtomicUsize::new(0));
    rem.store(NUM_SPAWN, Relaxed);
    rt.block_on(async {
        for i in 0..NUM_SPAWN {
            let tx = tx.clone();
            let rem = rem.clone();

            task::spawn(async move {
                println!("current = {:?};i = {} ", current().id(), i);
                if 1 == rem.fetch_sub(1, Relaxed) {
                    tx.send(()).unwrap();
                }
            });
        }

        let _ = rx.recv().unwrap();
    });
}


fn rt() -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
}
