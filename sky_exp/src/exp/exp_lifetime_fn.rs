#[allow(dead_code)]
pub fn main_lifetime_fn() {
    test_1();
    test_2();
}

// 函数的什么周期
#[allow(dead_code)]
fn test_1() {
    let x = String::from("wwww");
    let y = String::from("wwwwwee");
    let test = longest(x.as_str(), y.as_str());
    println!("{}", test);
    println!("{}", x);
}

//结构体重的什么周期
fn test_2() {
    let x = String::from("wwww");
    let struct_var = A {
        name: &x,
    };
    println!("test_2 = {:#?}" ,struct_var);
}

#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



