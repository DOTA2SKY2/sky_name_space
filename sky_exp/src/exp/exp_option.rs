#[allow(dead_code)]
pub fn main_option() {
    test_1();
    // test_2();
    // test_3();
}

// Option 泛型
fn test_1() {
    let sky1: Option<i32> = Option::None;
    println!("Option::None = {:?}", sky1);

    let sky: Option<i32> = Option::Some(2);
    match &sky {
        Option::Some(x) => {
            println!("Option::Some = {}", x)
        }
        Option::None => {}
    }
    let sky1 = &sky.unwrap();
    println!("Option unwrap() = {}", sky1);
   let option = test_1_1(sky);
    println!("test_1_1  = {}", option.unwrap());
    println!("test_1_1  = {}", sky.unwrap());
}


//if let控制流 赋值再判断
#[allow(dead_code)]
fn test_2() {
    let sky: Option<i32> = Option::Some(2);
    if let Option::Some(x) = sky {
        println!("test_2 = {}", x)
    } else {
        println!("test_2 None")
    }
}

//if let控制流 赋值赋值不了就判断
#[allow(dead_code)]
fn test_3() {
    let sky: Option<i32> = Option::Some(2);
    if let Option::Some(4) = sky {
        println!("test_3 = {}", 2)
    } else {
        println!("test_3 None")
    }
}

/** 输入输出
*
*/
#[allow(dead_code)]
fn test_1_1(x: Option<i32>) -> Option<i32> {
    match x{
        None=>{
            None
        },
        Some(x1)=>{
            Some(x1)
        },
    }
}

//
// #[allow(dead_code)]
// fn test_1_2() -> Option<String> {
//     match x {
//         None=>{
//             None
//         },
//         Some(x1)=>{
//             Some(String::from("wwww"))
//         },
//     }
// }