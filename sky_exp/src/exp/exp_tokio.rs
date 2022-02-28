use std::thread;
use std::thread::{current, ThreadId};
use tokio;
use tokio::{fs};
use tokio::runtime;
use tokio_test::{assert_ok, assert_pending, task};
use tokio_test::assert_ready;
use anyhow::Result;
use tokio::time::{self, sleep, sleep_until, Duration, Instant};

use tokio::sync::oneshot;

pub fn sky_tokio() -> Result<()> {
    println!("sky_tokio ThreadId_id = {:?}", current().id());
    let runtime1 = runtime::Builder::new_multi_thread()
    // let runtime1 = runtime::Builder::new_current_thread()
        .enable_all()
        // .thread_stack_size(8 * 1024 * 1024)
        // .thread_stack_size(8 )
        .worker_threads(2)
        .max_blocking_threads(2)
        .build()?;

    runtime1.block_on(async move {
        exp_select2().await
    });
    thread::sleep(Duration::from_secs(5));
    // exp_select();
    Ok(())
}




// 初步定义为tokio::join!里面都是同个线程
async fn exp_join() {
    println!("sync_one_lit_expr_comma current().id() = {:?}", current().id());

    let foo = tokio::join!(async {
        println!("1 current().id() = {:?}",current().id());
        sleep( Duration::from_secs(1)).await;
        println!("{:?}",1);
        1
    },async {
        println!("1 current().id() = {:?}",current().id());
        println!("{:?}",2);
        2
    },);
    println!("{:?}", foo);
}

/// poll ->执行 async里面的函数 -> (遇到await)转 padding -> 要是await 都结束了转->Ready
/// poll个数跟await个数必须一致
async fn two_poll() {
    let (tx1, rx1) = oneshot::channel::<&str>();
    let (tx2, rx2) = oneshot::channel::<u32>();

    let mut join = task::spawn(async {
        tokio::join!(async {
           let x = rx1.await.unwrap();
            sleep(Duration::from_secs(1)).await;
            thread::sleep(Duration::from_millis(1));
            println!("rx1 ThreadId = {:?}",current().id());
            x
        }, async {
            let x = rx2.await.unwrap();
            thread::sleep(Duration::from_secs(1));
            println!("rx2 ThreadId = {:?}",current().id());
            x
        })
    });

    println!("poll = {:?}", join.poll());
    println!("poll is_woken = {:?}", join.is_woken());
    tx2.send(123).unwrap();
    println!("send is_woken = {:?}", join.is_woken());


    assert_pending!(join.poll());
    println!("poll is_woken = {:?}", join.is_woken());
    tx1.send("hello").unwrap();
    println!("poll is_woken = {:?}", join.is_woken());
    join.poll();

    sleep(Duration::from_secs(3)).await;
    let res = assert_ready!(join.poll());

    assert_eq!(("hello", 123), res);
}

// 用了select 相对于不用await了
async fn exp_select() {
    let foo = tokio::select! {
        foo1 = async {
             println!("select ThreadId = {:?}",current().id());
            1
        } => foo1,
    };
    println!("foo = {:?}", foo);
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

///
async fn multi_pin() {
    tokio::pin! {
        let f1 = one();
        let f2 = two();
    }

    (&mut f1).await;
    (&mut f2).await;
}

///tokio::select! 宏允许在多个异步计算中等待，并在单个计算完成后返回
/// 使用了两个 oneshot 通道。任何一个通道都可以先完成。select！ 语句在两个通道上等待，并将 val 与任务返回的值绑定。当 tx1 或 tx2 完成时，相关的块被执行。
/// 没有完成的分支被放弃。在这个例子中，计算正在等待每个通道的 oneshot::Receiver。尚未完成的通道的 oneshot::Receiver 被放弃
async fn exp_select1() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}


async fn some_operation() -> String {
    // Compute value here
    thread::sleep(Duration::from_secs(2));
    String::from("some_operation")
}

/// tokio::select! 当有一个await完成，就丢弃其他await.
/// tx1 被丢弃只能在  多线程模式进行 runtime::Builder::new_multi_thread()
async fn exp_select2() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        println!("4444");
        let _ = tx2.send("two");
    });

    tokio::spawn(async {
        // Select on the operation and the oneshot's
        // `closed()` notification.
        println!("11111111");
        thread::sleep(Duration::from_secs(2));
        println!("2222222");
        tokio::select! {
            val = some_operation() => {
                println!("55555");
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("tx1.closed()");
                // `some_operation()` is canceled, the
                // task completes and `tx1` is dropped.
            }
        }
    });



    tokio::select! {
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
        val = rx1 => {

            println!("rx1 completed first with {:?}", val);
        }

    }

}