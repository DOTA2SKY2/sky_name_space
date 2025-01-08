#[derive(Debug, Clone)]
struct StructA {
    pub a: i32,
}

trait TraitA: Sized + Clone {
    fn hello(&self);
    fn new_obj(&self) -> Self {
        self.clone()
    }
}

impl TraitA for StructA {
    fn hello(&self) {
        println!("{:?}", self.a);
    }
}

pub fn main() {
    let sky = StructA {
        a: 1
    };
    let kk1 = sky.new_obj();
    kk1.hello();
    let kk2 = sky.new_obj();
    kk2.hello();
}
