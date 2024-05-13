#[allow(dead_code)]
fn main() {
    // fn_no();
    // fn_1(10, 20);
    // fn_2(11, 21);
    // fn_3(12, 22);
    fn_4();

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

/**
* create by sky at 2020/10/17 1:44 下午
* description: 闭包 闭包外的变量捕获借用，不获取他的所有权
* @param
* @return
*/
fn fn_4(){
    let a =  vec![1];
    let bi_bao = |x|{x == a}; //a的所有权没被挪走
    // let bi_bao = move |x|{x == a}; //a的所有权挪走
    println!("{:?}",a);
    let b = vec![1];
    assert!(bi_bao(b));
}

/**
* create by sky at 2020/10/17 1:44 下午
* description: 闭包，闭包的参数输入，转移他的所有权
* @param
* @return
*/

fn fn_5(){
    let a =  vec![1];
    let bi_bao = |x|{x == a};
    let b = vec![1];
    assert!(bi_bao(b)); //b的所有权被挪走了
    // println!("{:?}",b);
}








