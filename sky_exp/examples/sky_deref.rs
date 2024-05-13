use std::ops::Deref;

fn main() {
    test();
}

fn test() {
    struct DerefExample<T> {
        value: T,
        value1: T,
    }
    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = DerefExample {
        value: 'a',
        value1: 'b',
    };
    println!("{:?}", x.deref());
    println!("{:?}", *x);
}