use std::rc::Rc;
use std::sync::{Arc, mpsc};
// use std::ops::Deref;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    test6()
}


//结构体的属性所有权被移走，该结构体的其他属性是可以用的，被drop属性没有内存了，要重新赋值才行
fn test1() {
    #[derive(Debug)]
    struct A {
        a1: i32,
        a2: B,
    }

    #[derive(Debug)]
    struct B {
        b1: i32,
        b2: String,
    }

    fn moveto(b: B) {
        println!("b.b = {:?} ", b.b1);
        println!("b.h= {:?} ", b.b2);
    }
    //--------------------
    let bb = B {
        b1: 1,
        b2: String::from("struct B b"),
    };
    let a = A {
        a1: 1,
        a2: bb,
    };
    moveto(a.a2); // == drop(a.b)
    println!("{}", a.a1)
}

//rc
fn test2() {
    #[derive(Debug)]
    struct A {
        a1: i32,
        a2: Rc<B>,
    }
    #[derive(Debug)]
    struct B {
        b1: i32,
        b2: String,
    }
    fn moveto(b: Rc<B>) {
        println!("b.b = {:?} ", b.b1);
        println!("b.h= {:?} ", b.b2);
    }

    //--------------------
    let bb = B {
        b1: 1,
        b2: String::from("struct B b"),
    };
    println!("bb = {:?} ", bb);
    let mut a = A {
        a1: 1,
        a2: Rc::new(bb),
    };
    moveto(a.a2);
    println!("a.c = {:?} ", a.a1);
    let bb1 = B {
        b1: 2,
        b2: String::from("struct B b"),
    };
    a.a2 = Rc::new(bb1);
    println!("a.b = {:?} ", a.a1);
}


// Rc 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点
// 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
// 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
// Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
// Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。
fn test3() {
    let ll = String::from("clone");
    let str = Rc::new(ll); // 是类 不能copy
    let s1 = str.clone();
    let s2 = s1.clone();

    println!("1= {:p}",s1);
    println!("1= {:?}",s1.as_ptr());
    println!("2= {:p}",s2);
    drop(s1);
    println!("1= {:p}",s2);
}

// unspecified -> Display
// ? -> Debug
// o –> Octal //8进制
// x –> LowerHex //16进制
// X -> UpperHex
// p –> Pointer
// b –> Binary //二进制
// e -> LowerExp
// E -> UpperExp
fn test4() {
    let str = String::from("clone");
    let str1 = &str; // 是存int copy
    let ref str2 = str; // 是存int copy
    println!("1= {:p}",&str);
    println!("2= {:?}",&str.as_ptr());
    println!("2= {:?}",str1.as_ptr());
    println!("3= {:p}",str1);
}

fn test5() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            println!("sss ={:?}",local_numbers);
            // Work with the local numbers
        });
    }
    thread::sleep(Duration::from_secs(5));
}



#[allow(dead_code)]
fn test6() {
    static NTHREADS: usize = 30;
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送
    // 消息的类型（类型标注是可有可无的）
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let sky  = Arc::new(String::from("1111"));
    for id in 0..NTHREADS {
        // sender 发送端可被复制
        let _thread_tx = tx.clone();
        let _sky_clone = sky.clone();
        // 每个线程都将通过通道来发送它的 id
        thread::spawn(move || {
            // let sk =skyClone
            // thread_tx.send(skyClone).unwrap();
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    // let mut ids = Vec::with_capacity(NTHREADS);
    let mut ids = Vec::<String>::new();
    for _ in 0..NTHREADS {
        // `recv` 方法从通道中拿到一个消息
        // 若无可用消息的话，`recv` 将阻止当前线程
        let sky = rx.recv().unwrap();
        ids.push(sky);
    }

    // 显示已发送消息的次序
    println!("{:?}", ids);
}

