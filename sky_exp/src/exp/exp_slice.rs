
#[allow(dead_code)]
pub fn main_slice() {
    test_1();
}

fn test_1() {
    let str = String::from("abcdefg");
    println!("test_1 [0..0] = {}", &str[0..=0]);
    println!("test_1 [0..4] = {}", &str[0..4]);
    println!("test_1 [0..=4] = {}", &str[0..=4]);
    println!("test_1 [..=4] = {}", &str[..=4]);
    println!("test_1 [..=4] = {}", &str[..5]);


    println!("test_1 [..=4] = {}", &str[1..]);
    println!("test_1 [..=4] = {}", &str[..]);

    //中文报错，不是字节边界的话
//    let str = String::from("你好");
//    println!("test_1 [..=4] = {}", &str[1..]);
}