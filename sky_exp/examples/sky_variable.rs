#[allow(dead_code)]
fn main() {
    test_6()
}


/// 结构体赋值 逐个赋值
fn test_1() {
    #[derive(Debug)]
    struct User {
        name: String,
        count: i32,
        age: i32,
    }
    let st = User {
        name: String::from("User1"),
        count: 1,
        age: 4,
    };
    println!("test_1 st = {:?}", st);
}

/// 结构体赋值 匿名赋值
fn test_2() {
    #[derive(Debug)]
    struct User {
        name: String,
        count: i32,
        age: i32,
    }
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
    // println!("test_2 st = {:?}", st);

    let st = User {
        name: String::from("User1"),
        count: 1,
        age: 4,
    };
    //克隆就不会被报错了
    let st1 = User {
        name: st.name.clone(),
        ..st
    };
    println!("test_2 st1 = {:?}", st);
}


/// 解构式赋值,跟常用赋值不一样
fn test_5() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];

    struct Struct {
        e: i32,
    }
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}


fn test_6() {
    let l1 = vec![1u8;10];
    let l2 = vec![1u32;10];
    let l3 = vec![1u64;10];
    let l4 = vec![1i32;10];
    let l5 = vec![1i64;10];
    l1.iter().zip(l2.iter()).zip(l3).zip(l4).map(
        |item1|{

        }
    )

}


