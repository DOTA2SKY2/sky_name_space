// use std::any::Any;

#[allow(dead_code)]
fn main() {
    // test_1();
    // test_2();
    // test_3();
    test_4();
}


//无类型
#[derive(Debug)]
enum UserEnum {
    Name,
    Count,
}

//有类型
#[derive(Debug)]
enum UserEnumType {
    Quit,
    Name(String),
    Count(i32),
}

impl UserEnumType {
    fn show_name(&self) {
        match &*self {
            UserEnumType::Name(x) => {
                println!("ShowName = {}",x)
            }
            UserEnumType::Count(y) => {
                println!("ShowName = {}",y)
            }
            _ => {}
        }
    }
}

//有类型
//#[derive(Debug)]
//enum MessageEnum {
//    Name,
//    Move { x: i32, y: i32 },
//    Change(i32, i32),
//}



//无类型
fn test_1() {
    let user = UserEnum::Name;
    let count = UserEnum::Count;
    println!("user = {:?}", user);
    println!("count = {:?}", count);
}

//有类型
fn test_2() {
    //let user = UserEnumType::Name(String::from("www"));
    let user = UserEnumType::Quit;
    match user {
        UserEnumType::Quit=>{
            println!("user = {:?}", UserEnumType::Quit);
        }
        _ => {}
    }
//    let user1 = 1;
//    match user1 {
//        1=>{
//            println!("user1 = {:?}", 1);
//        }
//        _ => {}
//    }
    let count = UserEnumType::Count(3);
    println!("user = {:?}", user);
    println!("count = {:?}", count);
}

//有类型
fn test_3() {
    println!("user = {:?}", UserEnumType::Name(String::from("www")).show_name());
    println!("user = {:?}", UserEnumType::Count(6666).show_name());
}



//有类型
fn test_4() {
   let a = UserEnum::Count;
    let b = UserEnum::Count;

    dbg!(a as i32);
    dbg!(b as i32);
    // dbg!(b.type_id());
    // println!("{:?}",a.into());
    // println!("{:?}",b.into());

    // if a.type_id() {
    //     println!("相等");
    // }else{
    //     println!("相等");
    // }
}


