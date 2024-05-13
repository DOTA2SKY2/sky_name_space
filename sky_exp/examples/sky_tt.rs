#[allow(dead_code)]
fn main() {
    test_1();
}

// Option 泛型
fn test_1() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let number_list = vec!["34", "50", "25", "100", "65"];
    // let result = largest::<i32>(&number_list);
    let result = test11::largest1::<i32>(&number_list);
    println!("The largest number is {}", result);
}

#[allow(dead_code)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub mod test11{
    pub fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
}


