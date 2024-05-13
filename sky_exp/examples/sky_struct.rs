#[allow(dead_code)]
fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
}

#[derive(Debug)]
struct User {
    name: String,
    count: i32,
    age: i32,
}

//1 - 结构体赋值 逐个赋值
fn test_1() {
    let st = User {
        name: String::from("User1"),
        count: 1,
        age: 4,
    };
    println!("test_1 st = {:?}", st);
}

//1 - 结构体赋值 匿名赋值
fn test_2() {
    let name = String::from("User1");
    let st = User {
        name,
        count: 1,
        age: 4,
    };
    println!("test_2 st = {:?}", st);

    //赋值属性的所有权被转移
    let st1 = User {
        ..st
    };
//报错
//    println!("User.name = {}", st.name);
//    println!("User.count = {}", st.count);
//    println!("User.age = {}", st.age);
    println!("test_2 st1 = {:?}", st1);

    let st = User {
        name: String::from("User1"),
        count: 1,
        age: 4,
    };
    //克隆就不会被报错了
//    let st1 = User {
//        name: st.name.clone(),
//        ..st
//    };

        let st1 = st;
//    println!("User.name = {}", st.name);
//    println!("User.count = {}", st.count);
//    println!("User.age = {}", st.age);
    println!("test_2 st1 = {:?}", st1);
}

//可修改
fn test_3() {
    let name = String::from("User1");
    let mut st = User {
        name,
        count: 1,
        age: 4,
    };
    st.name = String::from("xxxx");
    st.count = 100;
    st.age = 1221;
    println!("test_3 st = {:?}", st);
}

struct Useruple(String,i32);

//元组结构体
fn test_4() {
    let st = Useruple (String::from("Useruple"),2);
    let st1 =st;
    println!("User.name = {}", st1.0);
//    println!("User.count = {}", st1.0);
}