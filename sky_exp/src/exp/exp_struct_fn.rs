use std::option::Option::Some;

#[allow(dead_code)]
pub fn main_struct_fn() {

    test_1();
}

#[derive(Debug)]
struct B {
    str: Option<String>,
}

#[derive(Debug)]
struct UserFn {
    name: String,
    count: i32,
    age: i32,
    // b: Option<B>,
    b: Option<String>,


}

impl UserFn {
    fn add_one(&self) -> i32 {
        self.count + 1
    }
    fn add_two(&self) -> i32 {
        self.count + 1
    }
}


fn ttt(bB:String){
    println!("b = {:?}", bB);



}

//1 - 结构体赋值 逐个赋值
fn test_1() {
    let mut st = UserFn {
        name: String::from("User1"),
        count: 1,
        age: 4,
        // b:Some(B{
        //     str:String::from("wwww")
        // })
        b:Some(String::from("www"))
    };
    //
    // let str1 = String::from("nnnnn");
    // let aaa = str1;
    // println!(str1);
    // let kkk = aaa;
    // ttt(kkk);


    // let jjjj = st.b;
    //
    // if let Some (aa) = jjjj{
    //    let kk = aa;
    //     // st.b = None;
    //     // st.b = None;
    //     ttt(kk)
    // }
    // st.b = None;
    //drop(st.b);
    // let kk = st.b;
    // ttt(kk);


    // drop(st.b);
    // st.b = Some(String::from("llll"));
    //  println!("test_1 st = {:?}", st);
    // println!("test_1 st add_one = {}", st.add_one());
    // println!("test_1 st add_two = {}", st.add_two());
}
