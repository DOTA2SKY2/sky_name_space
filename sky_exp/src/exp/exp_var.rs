//变量
#[allow(dead_code)]
pub fn main_var() {
    hello();
    hello_mut();
    hello_hide();
    hello_const();
}

//变量
fn hello() {
    let a:i32 = -1;
    println!("Hello, world!{}", a);
}

//可以修改的变量
fn hello_mut() {
    let mut a = 1;
    println!("修改前 = {}", a);
    a = 2;
    println!("修改后 = {}", a);
}

//变量的隐藏性
fn hello_hide() {
    let a = 1;
    println!("修改前 = {}", a);
    let a = String::from("aaa");
    println!("修改后 = {}", a);
}

//常量
fn hello_const() {
    const A: i32 = 100;
    println!("常量 = {}", A);
}








