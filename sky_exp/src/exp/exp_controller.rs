#[allow(dead_code)]
pub fn main_controller() {
    controller_if1();
    controller_if2();
    controller_loop();
    controller_while();
    controller_for();
}

fn controller_if1() {
    let a = 1;
    if a == 1 {
        println!(" controller_1 = {}", a);
    } else if a == 2 {
        println!(" controller_2 = {}", a);
    }
}


fn controller_if2() {
    let a = 1;
    let b = if a == 1 {
        String::from("a1")
    } else if a == 2 {
        String::from("a2")
    } else {
        String::from("a3")
    };
    println!("controller_if1 {}", b)
}

//loop 死循环
fn controller_loop() {
    let mut cnt = 1;
    //有返回值
    let res = loop {
        if cnt > 10 {
            break cnt;
        } else {
            println!("loop cnt = {}", cnt);
            cnt += 1;
        }
    };
    println!("controller_loop end = {} ", res);
}


//while 死循环
fn controller_while() {
    let mut cnt = 1;
    //有返回值
    while cnt <= 10 {
        println!("loop cnt = {}", cnt);
        cnt += 1;
    };
    println!("controller_while end = {} ", cnt);
}

//for 死循环
fn controller_for() {
    let arr: [i32; 5] = [0, 11, 22, 33, 44];
    for element in arr.iter() {
        println!("controller_for element = {} ", element);
    }
    for element in &arr {
        println!("controller_for element = {} ", element);
    }
}







