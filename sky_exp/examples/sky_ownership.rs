#[allow(dead_code)]
fn main() {
    owner_ship_1()
}

fn owner_ship_1() {
    //值类型的复制没有所有权
    let a = 1;
    let b = a;
    println!("owner_ship_1 a = {}", a);
    println!("owner_ship_1 a = {}", b);


    //字符串指针复制所有权移交
    let a = String::from("www");
    let b = a;      //a被释放了
//    println!("owner_ship_1 a = {}", a);
    println!("owner_ship_1 a = {}", b);

    //字符串克隆所有权移交
    let a = String::from("clone");
    let b = a.clone();      //a克隆不释放
    println!("owner_ship_1 a = {}", a);
    println!("owner_ship_1 a = {}", b);

    let a = String::from("copy");
    copy(a.clone()); //可以克隆个过去
//    copy(a); //所有权被里面用了，不是clone过去的，是指针引用进去的
    println!("copy end = {}", a);

    let a = String::from("copyPrt");
    copy_prt(&a); //引用过去
    println!("copy end = {}", a);


    //引用被修改
    let mut a = String::from("push_str");
    a.push_str("xxxx");
    copy_prt(&a); //引用过去
    println!("push_str end = {}", a);


    //引用转移
    let mut a = String::from("push_str");
    a.push_str("xxix");
    let b = &a;
    copy_prt(b); //引用过去
    println!("push_str end = {}", a);

    //引用被修改
    let mut a = String::from("push_str");
    let b = &mut a;
    copy_prt_mut( b);
    println!("引用被修改 end = {}", a);

}

//转移所有权
fn copy(a: String) {
    println!("copy = {}", a)
}


//引用 地址是值类型，不会被销毁
fn copy_prt(a: &String) {
    println!("copy_prt = {}", *a);  //a可以打印
    println!("copy_prt = {}", a);   //*a也可以打印
}

//引用 地址是值类型，不会被销毁
fn copy_prt_mut(a: &mut String) {
    a.push_str("_copy_prt_mut");
    println!("copy_prt = {}", *a);  //a可以打印
}



