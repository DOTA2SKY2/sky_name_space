use std::thread::current;

/// 通过 join 执行 Future join则会并发，不过都是当前线程id
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("main = {:?}", current().id());
    let f1 = function1();
    let f2 = function2();

    // 使用await则会顺序执行，
    // f1.await;
    // f2.await;

    //使用join则会并发执行f1和f2
    tokio::join!(f1, f2);
    // main = ThreadId(1)
    // function1 = ThreadId(1)
    // function2 = ThreadId(1)
    // function1 ++++
    // function2 ++++
}


async fn function1() {
    println!("function1 = {:?}", current().id());
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; //等待一秒钟
    println!("function1 ++++ ");
}

async fn function2() {
    println!("function2 = {:?}", current().id());
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; //等待一秒钟
    println!("function2 ++++ ");
}


