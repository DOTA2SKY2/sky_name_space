// use crate::exp::exp_thread::main_thread;
use crate::exp::exp_arc::main_arc;

pub mod exp;
fn main() {

    fil_logger::init();
    main_arc();
}
