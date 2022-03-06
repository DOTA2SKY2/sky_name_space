use std::time::Duration;
use tokio::{pin};


#[tokio::main]
async fn main() {
    exp_2().await
}


async fn my_async_fn() {
    // async logic here
    sleep(Duration::from_secs(4)).await;
    println!("my_async_fn");
}

/// 要是使用&mut future的情况, 必须用pin
/// 不用可改是不用pin
async fn exp_1() {
    let mut future = my_async_fn();
    /// 要是使用&mut future的情况, 必须用pin
    pin!(future);
    (&mut future).await;
}


/// 编译错误
// async fn exp_2() {
//     let mut future = pin!(my_async_fn());
//     (&mut future).await;
// }


use tokio::select;
use tokio::time::sleep;
use tokio_stream::{self as stream, StreamExt};

/// tokio_stream next有实现 future功能，可以用select!开启并发
async fn exp_2() {
    let mut stream = stream::iter(vec![1, 2, 3, 4]);

    let future = my_async_fn();
    pin!(future);

    loop {
        select! {
            _ = &mut future => {
                // Stop looping `future` will be polled after completion
                break;
            }
            Some(val) = stream.next() => {
                println!("got value = {}", val);
            }
        }
    }
}