use std::cell::RefCell;

pub fn exp_refcell() {
    let value = Data::default();
    println!("st1 = {:?}", value);
    value.value_b(1);
    println!("st2 = {:?}", value);
    value.value_b(2);
    println!("st3 = {:?}", value);
}


#[derive(Debug, Default)]
pub struct Data {
    pub a: u8,
    pub b: RefCell<u8>,
}

impl Data {
    // 编译通过
    pub fn value_b(&self, h: u8) -> u8 {
        //借用是可以更改值的
        let mut ref_mut = self.b.borrow_mut();
        *ref_mut = h;
        *ref_mut
    }

    //编译错误：cannot mutably borrow field of immutable binding
    // pub fn value_a(&self) -> u8 {
    //     if self.a != 0 {
    //         return self.a;
    //     }
    //
    // self.a = 100;
    // self.a
    // }
}



