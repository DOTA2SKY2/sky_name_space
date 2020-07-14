#[allow(dead_code)]
pub fn main_fn() {
    fn_no();
    fn_1(10, 20);
    fn_2(11, 21);
    fn_3(12, 22);
}

//变量
fn fn_no() {
    let a = 1;
    println!("Hello, world!{}", a);
}

//带参数函数
fn fn_1(a: i32, b: i32) {
    println!(" main_fn fn_1 = {}", a + b);
}

//带参数函数
fn fn_2(a: i32, b: i32) -> i32 {
    let res = a + b;
    println!(" main_fn fn_2 = {}", res);
    res
}

// {大括号在执行并在最后一个值返回值}
fn fn_3(a: i32, b: i32) -> i32 {
    let res = {
        a + b
    };
    println!(" main_fn fn_3 = {}", res);
    res
}







