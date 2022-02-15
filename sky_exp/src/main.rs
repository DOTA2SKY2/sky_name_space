// use crate::exp::exp_option::main_option;



use crate::exp::exp_fn::main_fn;
use crate::exp::exp_file_open::main_file_open;
use crate::exp::exp_vec::main_exp_vec;

pub mod exp;

type KKK = u16;

fn main() {
    // fil_logger::init();
    // main_file_open();
    // main_exp_vec();
    // main_thread();
    // main_max_thread();
    //  Some((quota_us as f64 / period_us as f64).ceil() as usize)
    //   let a:f32 = 61537282.0;
    //   let b:f32 = 8704.0;
    //   let c =   a / b;
    //   let c1 = c.ceil() as usize;
    //   println!("c = {:?}",c);
    //   println!("c1 = {:?}",c1);
    //   main_fn();
    //   main_exp_vec();


    // let a= sss{
    //     buf:11
    // };
    // let b= a;
    // println!("{:?}",a);
    // println!("{:?}",b);
    // let s :KKK= 18;
    // println!("{:?}",s)

    // let mut s1 = String::from("hello");
    // foo(s1);
    // let mut s2 = String::from("hello world");
    // s1 = s2;
    // println!("{}", s1);

    let x = &temp(); // temp 函数调用是一个无效的位置表达式
    let y = *x ;    // error[E0070]:invalid left-hand side expression
    println!("{:p}", x);
    println!("{:p}", &temp());
}

pub fn temp() -> i32 {
    return 1;
}

// fn temp() -> i32 {
//     1
// }

fn foo(s: String) {
    println!("{}", s);
}

#[derive(Debug, Clone, Copy)]
pub struct sss {
    buf: i32,
}