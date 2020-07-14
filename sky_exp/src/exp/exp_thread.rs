use std::thread;
use std::sync::{Mutex, Arc, mpsc};
use std::time::Duration;

#[allow(dead_code)]
pub fn main_thread() {
    // test1();
    // test2();
    test4();
}

/**
* 线程所有权
*
*/
fn test() {

    //如果我们在多个线程中有 data 的引用，线程拿走了引用的所有权，我们就有了三个拥有者了。这是不行的
    // let mut data=vec![1u32, 2, 3];

    // 你看看， Mutex 由一个 lock 方法，方法的签名是：
    //
    // fn lock(&self) ->LockResult<MutexGuard<T>>
    // 因为   MutexGuard<T>没有实现Send,我们不可以不能在线程之间传输这个对象，所以报错。
    // let mut data=Mutex::new(vec![1u32, 2, 3]);

    let data = Arc::new(Mutex::new(vec![0; 10]));
    for i in 0..10 {
        // let data=data.lock().unwrap();//没有实现Send
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 9;
            println!(" data = {:?}", data);
            //解引用
            // let mut da:[i32;3] = [1;3];
            // data.iter().enumerate().for_each(|v|{
            //     da[v.0] = *v.1; //解引用
            // });
            // dbg!(da);

            // let sky = data.iter().map(|v|{
            //     *v +1
            //  }).collect::<Vec<i32>>();
        });
    }
    thread::sleep(Duration::from_secs(2))
    // thread::sleep_ms(50);
}

/** 锁同步
* test2同个线程 Sender 与Receiver 是互相阻塞的
*
*/
fn test1() {
    let data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            // thread::sleep(Duration::from_secs(10 - i));
            let mut data = data.lock().unwrap();
            *data += 1;
            println!("第{:?}次：发送前#####  = {:?}", i, *data);
            tx.send((*data)); //不同线程不阻塞，
            println!("第{:?}次：发送后##### ", i);
        });
    }
    // thread::sleep(Duration::from_secs(8))
    for i in 0..11 {
        println!("第{:?}次：获取前@@ ", i);
        let sky = rx.recv().unwrap(); //无值阻塞。等send值
        println!("第{:?}次：获取后@@ ={:?}", i, sky);
        thread::sleep(Duration::from_secs(2));
    }
}

/** 同步机制 - 同个线程
* test2同个线程 Sender 与Receiver 是互相阻塞的
*
*/
fn test2() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) =
        mpsc::channel();
    let tx1 = tx.clone();
    // 创建线程用于发送消息
    thread::spawn(move || {
        // 发送一个消息，此处是数字id
        thread::sleep(Duration::from_secs(1));
        tx1.send(1).unwrap();
        println!("发送前#####  = {:?}", 1);
        thread::sleep(Duration::from_secs(1));
        tx1.send(2).unwrap();
        println!("发送前#####  = {:?}", 2);
        thread::sleep(Duration::from_secs(1));
        tx1.send(3).unwrap();
        println!("发送前#####  = {:?}", 3);
        thread::sleep(Duration::from_secs(1));
        tx1.send(4).unwrap();
        println!("发送前#####  = {:?}", 4);
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive1 {}", rx.recv().unwrap());
    thread::sleep(Duration::from_secs(2));
    println!("receive2 {}", rx.recv().unwrap());
    thread::sleep(Duration::from_secs(2));
    println!("receive3 {}", rx.recv().unwrap());
}

/** 异步机制
*/
fn test3() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // 创建线程用于发送消息
    for id in 0..10 {
        // 注意Sender是可以clone的，这样就可以支持多个发送者
        let thread_tx = tx.clone();
        thread::spawn(move || {
            // 发送一个消息，此处是数字id
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }


    println!("wake up");
    // 在主线程中接收子线程发送的消息并输出
    for _ in 0..10 {
        println!("receive {}", rx.recv().unwrap());
    }
}


/**
* tx销毁后 rx就会自动退出
* 其中一个tx克隆体或者自己存在，rx获取数据也要等；
* 所有克隆体与自己都销毁，rx就会自动退出
*/
fn test4() {
    let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // drop(tx1);
    // let tx2 = tx1.clone();
    // drop(tx);
    thread::spawn(move || {
        let val_s = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in val_s {
            tx.send(val).unwrap();
            tx.send(String::from("sss")).unwrap();
            thread::sleep(Duration::from_secs(6));
        }
        // tx2.send(String::from("tttt")).unwrap();
    });
    loop {
        let val = rx.try_recv();
        if let Ok(res) = val{
            dbg!(res);
           // thread::sleep(Duration::from_secs(2))
        }else {

            dbg!("为空");
            thread::sleep(Duration::from_secs(1));
        }
    }
    // for received in rx.iter() {
    //     println!("Got: {}", received);
    // }
    // dbg!("结束");
}


