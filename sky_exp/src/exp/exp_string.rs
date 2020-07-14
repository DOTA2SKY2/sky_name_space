#[allow(dead_code)]
pub fn main_string() {
    owner_string_a();
}

fn owner_string_a(){
//    int_str_copy();
//    string_copy();

    let sky_str = String::from("whew");
//    string_in(sky_str);
//   let sky_str2 = string_in_out(sky_str);
//    {
//        println!("传入函数被调用后：{}", sky_str);
////        string_in(sky_str);
//    }
    println!("传入函数被调用后：{}", sky_str);
}

//硬编码到二进制文件中 - 没有使用权
#[allow(dead_code)]
fn int_str_copy(){
    let str = "www";
    let str_copy = str;
    println!("str = {}",str);
    println!("str_copy = {}", str_copy);
}

//对象- 没有使用权
#[allow(dead_code)]
fn string_copy(){
    let string_1  = String::from("www");
    let string_copy = string_1;
//    println!("string_1 = {}",string_1);
    println!("string_copy = {}", string_copy);
}

//对象- 使用权 参数输入
#[allow(dead_code)]
fn string_in(str :String){
    println!("slsllll = {}",str);
}

//对象- 使用权 参数输入
#[allow(dead_code)]
fn string_in_out(str :String)->String{
    println!("slsllll = {}",str);
    str
}



