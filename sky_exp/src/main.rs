#![feature(thread_id_value)]
// use std::env;


// use lib1;

// use crate::exp::exp_vec::main_exp_vec;

// use crate::exp::exp_struct_fn::main_struct_fn;



// use crate::exp::exp_hashmap::main_hashmap;

// use crate::exp::exp_json::main_json;
// use crate::exp::exp_hashmap::main_hashmap;
// use crate::exp::exp_file_lock::main_file_lock;
// use crate::exp::exp_painc::main_painc;
// use crate::exp::exp_file_lock::main_file_lock;
// use crate::exp::exp_file_lock_json::main_file_lock_json;
// use crate::exp::exp_vec::main_exp_vec;

use crate::exp::exp_drop::main_drop;
use crate::exp::exp_file_lock_json::main_file_lock_json;
use crate::exp::exp_build_file::main_build_file;

pub mod exp;

fn main() {
    // main_file_lock_json();
    // main_hashmap();
    // main_exp_vec();
    // main_file_lock_json();
    main_build_file();
}
