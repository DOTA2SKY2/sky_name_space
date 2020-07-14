#[allow(dead_code)]
pub fn main_mod() {
    test_1();
}

mod sky_1 {
   pub mod sky_2 {
       pub  mod sky_3 {
           pub  fn print_3() {
                super::print_2();
                println!("print_3")

            }
        }

       pub fn print_2() {
            super::print_1();
            println!("print_2");
        }
    }
    pub fn print_1() {
        println!("print_1")
    }
}

// Option 泛型
fn test_1() {
    sky_1::sky_2::sky_3::print_3();
}



