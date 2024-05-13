use std::{thread, process};
use std::sync::{Mutex, Arc, mpsc, RwLock};
use std::time::Duration;
use rayon::prelude::*;
use std::thread::JoinHandle;
use rayon::ThreadPoolBuilder;

#[allow(dead_code)]
fn main() {
    // test1();
    // test2();
    // test4();
    // test8();
    cross_pool_busy1();
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
            tx.send((*data)).unwrap(); //不同线程不阻塞，
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
        if let Ok(res) = val {
            dbg!(res);
            // thread::sleep(Duration::from_secs(2))
        } else {
            dbg!("为空");
            thread::sleep(Duration::from_secs(1));
        }
    }
    // for received in rx.iter() {
    //     println!("Got: {}", received);
    // }
    // dbg!("结束");
}


fn test5() {
    let _it_sky:usize = 0;
    let nn :usize = 6;
    let _groth_proofs_list = (3..nn)
        .into_par_iter()
        .enumerate()
        .map(|(k, v)| {
            println!("进程 = {:?}; 线程 = {:?}；",process::id(), thread::current().id());
            println!("k = {:?}; v = {:?}；",k, v);
            // it_sky = 1;
            1
        })
        .collect::<Vec<_>>();

}

#[derive(Debug)]
struct Sky {
    a: i32,
    b: i32,
}

fn test6() {

    let data = RwLock::new(Sky{
        a: 0,
        b: 1,
    });

    // let data1 = data.clone();

    // for i in 0..10 {
    //     // let data=data.lock().unwrap();//没有实现Send
    //     let data = data.clone();
    //     thread::spawn(move || {
    //         let mut data = data.lock().unwrap();
    //         // let mut sky = *data;
    //         data.a = 100;
    //         data.b = 200;
    //
    //         println!(" data = {:?}", data);
    //
    //     });
    // }


    let _groth_proofs_list = (0..10)
        .into_par_iter()
        .enumerate()
        .map(|(k, v)| {

            let mut data1 = data.write().unwrap();
            data1.a = k as i32;
            data1.b = v as i32;

            thread::sleep(Duration::from_secs(1));
            println!("进程 = {:?}; 线程 = {:?}；k = {:?}；data1 = {:?}",process::id(), thread::current().id(), k, data1);
            drop(data1);

            // let mut data2 = data.read().unwrap();
            // // data2.a = k as i32;
            // // data2.b = v as i32;
            //
            // println!("进程 = {:?}; 线程 = {:?}；data2 = {:?}",process::id(), thread::current().id(),data2);
            // println!("进程 = {:?}; 线程 = {:?}；data2 = {:?}",process::id(), thread::current().id(),*data2);
            // drop(data2);
            // thread::sleep(Duration::from_secs(1));
            // println!("k = {:?}; v = {:?}；",k, v);
            // // it_sky = 1;
            // 1
        })
        .collect::<Vec<_>>();

    let _groth_proofs_list = (0..100)
        .into_par_iter()
        .enumerate()
        .map(|(_k, _v)| {
            // let mut data1 = data.write().unwrap();
            // data1.a = k as i32;
            // data1.b = v as i32;
            //
            //
            // println!("进程 = {:?}; 线程 = {:?}；data1 = {:?}",process::id(), thread::current().id(),data1);
            // drop(data1);
            let data2 = data.read().unwrap();
            // data2.a = k as i32;
            // data2.b = v as i32;

            println!("进程 = {:?}; 线程 = {:?}；data2 = {:?}",process::id(), thread::current().id(),data2);
            println!("进程 = {:?}; 线程 = {:?}；data2 = {:?}",process::id(), thread::current().id(),*data2);
            thread::sleep(Duration::from_secs(1));
            drop(data2);

            // println!("k = {:?}; v = {:?}；",k, v);
            // // it_sky = 1;
            // 1
        })
        .collect::<Vec<_>>();
    thread::sleep(Duration::from_secs(10));


}

fn test7() {

    let data_lock =Arc::new(RwLock::new(Sky{
        a: 0,
        b: 1,
    })) ;

    let mut  list:Vec<JoinHandle<()>>  = vec![];
    for i in 0..10 {
        let c = data_lock.clone();
      let sky = thread::spawn( move|| {
            thread::sleep(Duration::from_secs(1 * (i as u64) ));
          println!("进程 = {:?}; 线程 = {:?}；index = {:?}； write before",process::id(), thread::current().id(), i);
            let mut data = c.write().unwrap();
            thread::sleep(Duration::from_secs(2 * (i as u64) ));
            println!("进程 = {:?}; 线程 = {:?}；index = {:?}； write",process::id(), thread::current().id(), i);
            data.a = i;
            data.b = i;
          drop(data);
        });
        list.push(sky);
    }



    for i in 0..10 {
        let c = data_lock.clone();
        let sky = thread::spawn(move || {
            // thread::sleep(Duration::from_secs(1 ));
            println!("进程 = {:?}; 线程 = {:?}；index = {:?}；read before ",process::id(), thread::current().id(), i);
            let data = c.read().unwrap();
                println!("进程 = {:?}; 线程 = {:?}；index = {:?}；read ok data2 = {:?}",process::id(), thread::current().id(), i, data.a);
        });
        thread::sleep(Duration::from_secs(1 ));
        list.push(sky);
    }

    for item in list {
        item.join().unwrap()
    }



    // list.iter().map(|new_thread|{
    //     new_thread.join().unwrap();
    // });


    // 等待新建线程先执行
    // new_thread.join().unwrap();

}



fn test8() {

    let _data_lock =Arc::new(RwLock::new(Sky{
        a: 0,
        b: 1,
    })) ;
    // jj (&'static dataLock);

}

fn jj(lll : &'static RwLock<Sky>)
{
    let _sky = thread::spawn( move|| {
       let _uu = lll.clone();
    });
    // let s =lll;
}

fn cross_pool_busy() {
    let pool1 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    let pool2 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    let n: i32 = 100;
    let sum: i32 = pool1.install(move || {
        // Each item will block on pool2, but pool1 can continue processing other work from the
        // parallel iterator in the meantime. There's a chance that pool1 will still be awake to
        // see the latch set without being tickled, and then it will drop that stack job. The latch
        // internals must not assume that the job will still be alive after it's set!
        (1..=n)
            .into_par_iter()
            .map(|i| pool2.install(move || i))
            .sum()
    });
    assert_eq!(sum, n * (n + 1) / 2);
}
fn cross_pool_busy1() {
    let pool1 = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let n: i32 = 100;
    let sum: i32 = pool1.install(move || {
        // Each item will block on pool2, but pool1 can continue processing other work from the
        // parallel iterator in the meantime. There's a chance that pool1 will still be awake to
        // see the latch set without being tickled, and then it will drop that stack job. The latch
        // internals must not assume that the job will still be alive after it's set!
        (1..=n)
            .into_par_iter()
            .map(|i|{
                println!("进程 = {:?}; 线程 = {:?}；", process::id(), thread::current().id());
                i })
            .sum()
    });
    drop(pool1);
    assert_eq!(sum, n * (n + 1) / 2);
}





