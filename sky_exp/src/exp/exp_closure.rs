//变量
#[allow(dead_code)]
pub fn main_closure() {
    test1();

    // test2()
}
#[allow(dead_code)]
fn test() {
    let  num = 5;

    let plus_num = |x: i32| x + num;

    assert_eq!(10, plus_num(5));
}
#[allow(dead_code)]
fn test1(){
    let mut num = 5;
    {
        let plus_num = |x: i32| x + num;
        assert_eq!(10, plus_num(5));
    } // plus_num goes out of scope, borrow of num ends

    let y = &mut num;
    print!("plus_num = {}", y);
}


#[allow(dead_code)]
fn test2(){
    // #[derive(Debug)]
    // let nums = vec![1, 2, 3];

    // let takes_nums = move || nums;

    // println!("{:?}", nums);
    // println!("{}", takes_nums()[0]);
}