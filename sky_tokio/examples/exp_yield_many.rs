use std::sync::mpsc;
use std::thread::{current, sleep};
use std::time::Duration;
use tokio::runtime;
use tokio::runtime::Runtime;

/// 在异步编程下，多个任务执行，用 多个channel 在判断所有任务是否完成

fn main() {
    const NUM_YIELD: usize = 1_000;
    const TASKS: usize = 200;

    let rt = rt();

    let (tx, rx) = mpsc::sync_channel(TASKS);

    for _ in 0..TASKS {
        let tx = tx.clone();

        rt.spawn(async move {
            for _ in 0..NUM_YIELD {
                // println!("前current = {:?}; i = {}", current().id(),i);
                tokio::task::yield_now().await;
                // println!("后 current = {:?}; i = {}", current().id(),i);
            }
            tx.send(()).unwrap();
            println!("{:?}", current().id());
        });
    }

    for _ in 0..TASKS {
        sleep(Duration::from_secs(1));
        let _ = rx.recv().unwrap();
    }
}


fn rt() -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
}