#[allow(dead_code)]
pub fn main_var_type() {
    // var_type()
    test_1();
}

#[allow(dead_code)]
fn var_type() {
    let a: bool = true;
    println!("bool = {}", a);
//字符串
    let a: String = String::from("q's");
    println!("String = {}", a);
//整形
    let a: i8 = 11;
    println!("i8 = {}", a);

    let a: i16 = 11;
    println!("i16 = {}", a);
    let a: i32 = 1111111;
    println!("i32 = {}", a);
    let a: i64 = 1111111;
    println!("i64 = {}", a);
    let a: i128 = 1111111;
    println!("i128 = {}", a);
    let a: usize = 1111111;
    println!("usize = {}", a);
    //浮点型
    let a: f32 = 1111111.11;
    println!("f32 = {}", a);
    let a: f64 = 1111111.22;
    println!("f64 = {}", a);

//数组
    let a: [i32; 5] = [0, 11, 22, 33, 44];
    for value in &a {
        println!("数组打印 = {}", value);
    }

    //元组
    let a: (i32, bool, String) = (11, true, String::from("元组"));
    println!("元组.0 = {}", a.0);
    println!("元组.1 = {}", a.1);
    println!("元组.2 = {}", a.2);

    //元组
    let (x, y, z) = (11, true, String::from("元组"));
    println!("元组.x = {}", x);
    println!("元组.y = {}", y);
    println!("元组.z = {}", z);
}

fn test_1(){
    let sky :i64 = 222;
    println!("bn = {:x}", sky);
}