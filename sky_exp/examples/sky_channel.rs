/** 通道
*   匿名函数中的FnOnce/FnMut/Fn
*   首先 FnOnce/FnMut/Fn 这三个东西被称为 Trait,
*   默认情况下它们是交给rust编译器去推理的, 大致的推理原则是:
*       FnOnce: 当指定这个Trait时, 匿名函数内访问的外部变量必须拥有所有权.
*       FnMut: 当指定这个Trait时, 匿名函数可以改变外部变量的值.
*       Fn: 当指定这个Trait时, 匿名函数只能读取(borrow value immutably)变量值.
**/
use std::sync::mpsc::{Sender, Receiver};
use std::{thread};
use std::sync::mpsc;

#[allow(dead_code)]
fn main_channel() {
    test();
}

static NTHREADS: usize = 3;


/** 通道
*   在本线程开好 channel，负责收，跟其他线程发数据到本线程，发送端可以克隆被channel。
**/
#[allow(dead_code)]
fn test() {
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送
    // 消息的类型（类型标注是可有可无的）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // sender 发送端可被复制
        let thread_tx = tx.clone();
        // 每个线程都将通过通道来发送它的 id
        thread::spawn(move || {
            // 此线程取得 `thread_tx` 所有权
            // 每个线程都在通道中排队列出消息
            // （原文：The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel）
            thread_tx.send(id as i32).unwrap();

            // 发送是一个非阻塞操作，线程将在发送完消息后继续进行
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    let mut ids = Vec::with_capacity(NTHREADS);
    for _ in 0..NTHREADS {
        // `recv` 方法从通道中拿到一个消息
        // 若无可用消息的话，`recv` 将阻止当前线程
        ids.push(rx.recv());
    }

    // 显示已发送消息的次序
    println!("{:?}", ids);
}


fn main(){
}
