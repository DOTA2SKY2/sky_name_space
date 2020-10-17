
// use crate::exp::exp_option::main_option;



use crate::exp::exp_fn::main_fn;

pub mod exp;
fn main() {
    fil_logger::init();
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
    main_fn();
}
