use std::thread;
use std::thread::{current};
use std::time::Duration;
use tokio::sync::oneshot;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    exp_loop().await;
}


/// 通过 join 执行 Future join则会并发，不过都是当前线程ids
async fn exp_1() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    println!("main = {:?}", current().id());

    tokio::spawn(async {
        println!("tx2 = {:?}", current().id());
        let _ = tx2.send("two");
    });

    tokio::spawn(async {
        // Select on the operation and the oneshot's
        // `closed()` notification.
        println!("tx1 = {:?}", current().id());
        thread::sleep(Duration::from_secs(3));
        // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        tokio::select! {
            val = some_operation() => {
                println!("{:?}",val);
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

async fn some_operation() -> String {
    // Compute value here
    thread::sleep(Duration::from_secs(3));
    String::from("some_operation")
}


// tokio::select!执行的是await函数
async fn exp_2() {
    let out = tokio::select! {
        res1 = computation1() => res1,
        res2 = computation2() => res2,
    };

    println!("Got = {}", out);
}

async fn computation1() -> String {
    // .. computation
    return String::from("computation1");
}

async fn computation2() -> String {
    // .. computation
    return String::from("computation2");
}


use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_test::task::Spawn;


/// mpsc::channel跟oneshot::channel()不一样
/// 模式匹配
async fn exp_3() {
    let (mut tx1, mut rx1) = mpsc::channel::<i32>(2);
    let (mut tx2, mut rx2) = mpsc::channel::<i32>(2);

    tokio::spawn(async move {
        // thread::sleep(Duration::from_secs(1));
        sleep(Duration::from_secs(1)).await;
        // tx1.closed();
        // tx2.closed();
        tx1.send(111).await;
        // Do something w/ `tx1` and `tx2`
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
    };
}

///data 变量被从两个异步表达式中不可变地借用。当其中一个操作成功完成时，另一个就会被放弃。因为我们在 Ok(_) 上进行模式匹配，如果一个表达式失败，另一个表达式继续执行。
// 当涉及到每个分支的 <handler> 时，select！ 保证只运行一个 <handler>。正因为如此，每个<handler>都可以相互借用相同的数据。
// 例如，这在两个处理程序中都修改了out:
async fn exp_mut() {
    let (tx1, rx1) = oneshot::channel::<&'static str>();
    let (tx2, rx2) = oneshot::channel::<&'static str>();

    let mut out = String::new();

    tokio::spawn(async move {
        tx1.send("www");
        tx2.send("www1");
        // Send values on `tx1` and `tx2`.
    });

    tokio::select! {
        _ = rx1 => {
            out.push_str("rx1 completed");
        }
        _ = rx2 => {
            out.push_str("rx2 completed");
        }
    };

    println!("{}", out);
}



/// 循环
/// select! 宏经常在循环中使用。本节将通过一些例子来说明在循环中使用 select! 宏的常见方法。我们首先在多个通道上进行选择。
async fn exp_loop() {
    let (tx1, mut rx1) = mpsc::channel(1);
    let (tx2, mut rx2) = mpsc::channel(1);
    let (tx3, mut rx3) = mpsc::channel(1);

    let sender = tx1.clone();
    let sender2 = tx2.clone();
    let sender3 = tx3.clone();
    tokio::spawn(async move {
        sender.send("aa1").await;
        sender.send("aa1").await;
        sender.send("aa1").await;
        sender.send("aa1").await;
        sender2.send("aa2").await;
        sender3.send("aa3").await;
        // Send values on `tx1` and `tx2`.
    });
    tokio::spawn(async move {
        tx1.send("ww2").await;
        tx2.send("ww2").await;
        tx3.send("ww3").await;
        // Send values on `tx1` and `tx2`.
    });


    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {}", msg);
    }




    println!("All channels have been closed.");
}



async fn action() {
    sleep(Duration::from_secs(5)).await;
    println!("action")
    // Some asynchronous logic
}

/// 恢复异步操作
///现在我们将展示如何在多次调用 select! 时运行一个异步操作。在这个例子中，我们有一个MPSC通道，类型为i32，还有一个异步函数。我们想运行异步函数，直到它完成或在通道上收到一个偶数整数。s
async fn exp_loop1() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    let operation = action();
    tokio::pin!(operation);

    tokio::spawn(async move {
        tx.send(2).await
    });

    loop {
        sleep(Duration::from_secs(1)).await;
        println!("loop");
        tokio::select! {
            _ = &mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    println!("tx");
                    break;
                }
            }
        }
    }
}