/** 闭包
*   匿名函数中的FnOnce/FnMut/Fn
*   首先 FnOnce/FnMut/Fn 这三个东西被称为 Trait,
*   默认情况下它们是交给rust编译器去推理的, 大致的推理原则是:
*       FnOnce: 当指定这个Trait时, 匿名函数内访问的外部变量必须拥有所有权.
*       FnMut: 当指定这个Trait时, 匿名函数可以改变外部变量的值.
*       Fn: 当指定这个Trait时, 匿名函数只能读取(borrow value immutably)变量值.
**/
#[allow(dead_code)]
fn main_callback() {
    test();
}


#[allow(dead_code)]
fn test() {
    call_fn_callback();
}


/**  Fn Trait 匿名函数
*   若闭包的所有权没有被转移，默认实现了 Fn
**/
#[allow(dead_code)]
fn call_fn_test_fn_inline() {
    let b = String::from("hello");
    let pushed_data = || {
        println!("闭包读：b = {:?}", b);
    };
    pushed_data();
    println!("b = {}", b);
    pushed_data();          // 可以执行
    println!("b = {}", b);
}


/**  FnOnce Trait 匿名函数
*   闭包的所有权被转移，默认实现了 FnOnce Trait - move
**/
#[allow(dead_code)]
fn call_fn_test_fn_once_inline() {
    let b = String::from("hello");
    let pushed_data = move || {
        println!("move 闭包读：b = {:?}", b);
    };
    pushed_data();
    // println!("{}", b);     // error
}



/**  FnOnce Trait 匿名函数
*   闭包的所有权被转移，默认实现了 FnOnce Trait - return
**/
#[allow(dead_code)]
fn call_fn_test_fn_once_return() {
    let b = String::from("hello");
    let pushed_data = || {
        println!("return 闭包读：b = {:?}", b);
        b
    };
    let res = pushed_data();
    println!("res = {}", res);     // 这里只能运行一次.
    // println!("b = {}", b);     // error
}


/** FnOnce Trait 匿名函数
*  闭包的所有权被转移，默认实现了 FnOnce Trait - return
**/
#[allow(dead_code)]
fn call_fn_test_fn_once() {
    fn calculate<T>(callback: T) -> i32 where T: FnOnce() -> String
    {
        let data = callback();
        data.len() as i32
    }
    let mut y = String::from("hello");
    let result = calculate(|| {
        y.push_str(" world!");
        y
    });
    // println!("y = {}", y);
    println!("{}", result);
}


/** 函数闭包 - FnMut
*   1、闭包也是引用类型 FnMut - 作为参数的时候所有权会转移
*   2、FnMut闭包 - 只要不转移， 被捕获的外包变量，可以下个闭包使用
*/
#[allow(dead_code)]
fn call_fn_test_fn_mut() {
    fn calculate<T>(mut callback: T) where T: FnMut(),
    {
        callback()
    }
    let mut ba = 1;
    let ga = move || {
        ba = ba + 1
    };
    calculate(ga);
    println!("ba = {:?}", ba);
    calculate(|| {
        ba = ba + 1
    });
    println!("ba = {:?}", ba);

    let mut b = String::from("hello");
    let pushed_data = || {
        b.push_str(" hello!");
    };
    calculate(pushed_data);
    calculate(|| {
        b.push_str(" hello!");
    });

    calculate(|| {
        b.push_str(" hello!");
    });
    calculate(|| {
        b.push_str(" hello!");
    });

    println!("{}", b);
}


/** 闭包做形参，所有权被移走了
*
*/
#[allow(dead_code)]
fn call_fn_test_mut() {
    // fn call_Fn(x: i32, f: impl FnOnce(i32) -> i32) {
    //     f(x);
    //     println!("调用闭包");
    // }
    fn call_fn<F>(x: i32, mut f: F) where F: FnMut(i32) -> i32 {
        f(x);
        println!("调用闭包");
    }
    let mut j = String::from("str");
    let g = |i: i32| {
        j.push_str(" world!");
        println!("i32 = {:?}", i);
        println!("String = {:?}", j);
        i
    };
    call_fn(2, g);
    // call_fn(2, g); //g做形参，所有权被移走了
}


/** 闭包参数输入
*
*/
#[allow(dead_code)]
fn call_fn_input() {
    let mut a = String::from("hello");
    let b = String::from(" world!");
    let pushed_data = |x: &mut String| {
        // b 再这里被引用, 但是最后还能被打印, 证明它是被immutable引用.
        x.push_str(b.as_str());
        println!(" {}", x);
    };
    pushed_data(&mut a);
    pushed_data(&mut a);
    println!("{}", b);
}


/** 闭包参数输入
*
*/
#[allow(dead_code)]
fn call_fn_callback() {
    // fn calculate(f: impl Fn()) {
    //     f();
    //     println!("调用闭包");
    // }
    fn calculate<T>(callback: T) where T: Fn()
    {
        callback();
    }
    let a = String::from("hello-------");
    let b = String::from(" world!");
    calculate(|| {
        let joined = format!("{}{}", a, b);
        println!("{}", joined)
    });
    calculate(|| {
        let joined = format!("{}{}", a, b);
        println!("{}", joined)
    });
    calculate(|| {
        let joined = format!("{}{}", a, b);
        println!("{}", joined)
    })
}


/** 实际跟函数差不多 有move和无move
*   1、引用 String 所有权转移
*   2、&str 是常量 所有权不生效
*   3、值类型 i32   是复制值进去 所有权不生效
*/
#[allow(dead_code)]
fn call_fn_test_owner_ship() {
    let b_move = "hello".to_string();//
    let c_copy = "hello";//类同 2_u32,复制语义
    let d_copy = "world";//类同 2_u32
    let e_move = "world".to_string();//移动语义

    let b = move || b_move;
    let c = move || c_copy;
    let d = || d_copy;
    let e = || e_move;

    b();// 移动语义：move转移了所有权；b_move进了闭包内; 同时，外部的b_move不在。
//b();// error

    c();//
    c();//复制语义：move转移了所有权，c_copy进了闭包内；这是一种所有权转移的储蓄行为；但外部的c_copy还在；

    d();
    d();//d()还在; d_copy因为复制语义消费掉了，是一种消费行为；外部d_copy还在。

    e();//此后，e_move没了，e()没有了 ；=>FnOnce
}


/** 是一种特例，这个函数本身并不消费，也不转移。
*   1、引用 String 所有权转移
*   2、&str 是常量 所有权不生效
*   3、值类型 i32   是复制值进去 所有权不生效
*/
#[allow(dead_code)]
fn call_fn_test_owner_ship_println() {
    let m_move = "hello".to_string();
    let p_copy = "hello!";
    let q_copy = "hello world";//变量为复制语义类型
    let z_move = "hello world".to_string();//变量为移动语义类型

    let m = || { println!("m_move:{:?}", m_move); };
    let p = || { println!("p_copy:{:?}", p_copy); };
    let q = move || { println!("q_copy:{:?}", q_copy); };
    let z = move || { println!("z_move:{:?}", z_move); };

    m();
    m();//println!()无影响

    p();
    p();//p在，p_copy还在，println!()没有影响

    q();//q_copy留在q()内部，q_copy外部仍然还在,用的是copy；类似的有u32,i32等
    q();

    z();
    z();//z_move留在z()内部，z_move外部已经不在了；因为移动语义类型

    println!("p_copy:{:?}", p_copy);
    println!("q_copy:{:?}", q_copy);
    println!("m_move:{:?}", m_move);
}


/** 复制语义变量 ：变化 与有move、无move
*   &str 是常量 move 是复制值给到闭包
*   &str 是常量 no move 是用自己的地址
*/
#[allow(dead_code)]
fn call_fn_test_owner_ship_mut() {
    let mut s1 = "world"; //复制语义变量
    let mut s2 = "world";
    let mut f1 = || {
        println!("no move: f1 s1 in => {}", s1);
        s1 = "hello";
    };
    let mut f2 = move || {
        println!("   move: f2 s2 in => {}", s2);
        s2 = "hello";
    };
    f1();  //world
    f1();  //hello
    f1();  //hello
    f2();  //world
    f2();  //hello
    f2();  //hello
    println!("no move s1 : 原来是world,现在是： {} ", s1); //world=>hello
    println!("  move  s2 : 原来是world,现在是： {} ", s2); //world=>world
}


/** 移动语义：有move、无move
*   Vec mut 引用类型 有没有move都是转移
*/
#[allow(dead_code)]
fn call_fn_test_owner_ship_mut_move() {
    let mut v1: Vec<u32> = vec![];
    let mut v2: Vec<u32> = vec![];
    let v3: Vec<u32> = vec![];
    let g1 = || {
        v1.push(1);
        println!("no move v1 : {:?}", v1);
        v1
    };

    let g2 = move || {
        v2.push(1);
        println!("   move v2 : {:?}", v2);
        v2
    };
    let g3 = move || v3;
    g1();
//g1();//error
    g2();
//g2();//error
    g3();
//g3(); //error
}


/** 函数闭包做参数 - 2种
*   1、泛型
*   2、实现
*/
#[allow(dead_code)]
fn call_fn_test_i32() {
    fn call_fn(x: i32, f: impl FnOnce(i32) -> i32) {
        f(x);
        println!("调用闭包");
    }
    // fn call_Fn<F>(x: i32, f: F) where F: FnOnce(i32) -> i32 {
    //     f(x);
    //     println!("调用闭包");
    // }
    let j = 22;
    let g = |i: i32| {
        println!("i32 = {:?}", i);
        println!("j i32 = {:?}", j);
        i
    };
    call_fn(2, g);
    call_fn(2, g);
}


/** 函数闭包 - mut变量使用
*
*/
#[allow(dead_code)]
fn call_fn_test_string() {
    // fn call_Fn(x: i32, f: impl FnOnce(i32) -> i32) {
    //     f(x);
    //     println!("调用闭包");
    // }
    fn call_fn<F>(x: i32, mut f: F) where F: FnMut(i32) -> i32 {
        f(x);
        println!("调用闭包");
    }
    let mut j = String::from("str");
    let g = |i: i32| {
        j.push_str(" world!");
        println!("i32 = {:?}", i);
        println!("String = {:?}", j);
        i
    };
    call_fn(2, g);
    // call_fn(2, g);
}

fn main(){
}