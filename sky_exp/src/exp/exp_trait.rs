// static GIT_VERSION_FILE: &str = include_str!("git_version_file");
// use log::{info};

// #[macro_use]
// extern crate lazy_static;
# [derive(Debug,Clone)]
struct StructA {
    pub a: i32
}

trait TraitA: Sized + std::clone::Clone{
    fn hello(&self);
    fn new_obj(&self) -> Self {
        self.clone()
    }
}

impl TraitA for StructA {
    fn hello(&self) {
        println!("{:?}", self.a);
    }
    // fn new_obj(self) -> Self {
    //     return StructA {
    //         a: 222,
    //     };
    // }
}

pub fn main_trait() {
    let sky = StructA {
        a: 1
    };
    let kk1 = sky.new_obj();
    kk1.hello();
    let kk2 = sky.new_obj();
    kk2.hello();
}
