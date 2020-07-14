#[cfg(test)]
pub mod mod_test {
    #[test]
    pub fn main_test1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    pub fn main_test2() {
        assert!(true);
    }

    #[test]
    pub fn main_test3() {
        assert_ne!(2 + 2, 2);
    }
    #[test]
    pub fn main_test4() {
//        panic!("Make this test fail");
    }

    #[test]
    pub fn main_test5() {
        println!("main_test5 = {} ", "main_test5");
//        panic!("Make this test fail");
    }

//    #[test]
//    fn greeting_contains_name() {
//        let result = greeting("Carol");
//        assert!(
//            result.contains("Carol"),
//            "Greeting did not contain name, value was `{}`", result
//        );
//    }
//    pub fn greeting(name: &str) -> String {
//        String::from("Hello!")
//    }

    #[test]
    #[should_panic]
    fn main_test6() {
        main_test6_1();
    }

    fn main_test6_1() {
        panic!("main_test6_1 err");
    }

    #[test]
    pub fn main_test7() {
        println!("main_test7");
        assert!(true);
    }

    #[test]
    pub fn main_7() {
        println!("main_7");
        assert!(true);
    }

}





