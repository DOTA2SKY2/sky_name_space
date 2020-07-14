use std::ops::{AddAssign, MulAssign};
use std::str;
#[allow(dead_code)]
pub fn main_exp_vec() {
    // test_1();
    // test_2();
    // test_3();
    // test_4();
    // test_5();
    // test_6();
    // test_7();
    // test_8();
    // test_10();
    test_12();
}

#[allow(dead_code)]
fn test_1() {
    let v = vec![-2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    //skip(5)跳过第5标及之前的数，就是跳过5个数的意思
    let sky: Vec<_> = v.iter().skip(5).map(|x| {
        x
    }).collect();
    // let items1: Vec<_> = sky.collect();
    println!("wwww  = {:#?}", sky);
}

#[allow(dead_code)]
fn test_2() {
    let items = vec!["a", "bb", "ccc", "dddd", "eeeee"];
    let k = 3;
    //skip(5)跳过第5标及之前的数，就是跳过5个数的意思
    let one = items.iter().enumerate().skip(k).map(|(i, v)| (v.len(), i));
    println!("one = {:#?}", one);
    let two = items[k..].iter().enumerate().map(|(i, v)| (v.len(), i + k));
    println!("two = {:#?}", two);
    // Sanity check that the results are the same
    let items1: Vec<_> = one.collect();
    let items2: Vec<_> = two.collect();
    println!("items1  = {:?}", items1);
    println!("{}", items1 == items2);
}

#[allow(dead_code)]
fn test_3() {
    let a = [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let iter = a.iter().enumerate().skip(2).map(|(key, value)| {
        let sky = format!("key = {} ; value = {}", key, value);
        sky
    });
    let sky: Vec<String> = iter.collect();
    for item in sky.iter() {
        println!("item ={:#?}", item);
    }
}

#[allow(dead_code)]
fn test_4() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //zip输入是遍历器，多个zip就多了一元
    let sky: Vec<i32> = a.iter().zip(b.iter()).zip(b.iter()).map(|((x, y), z)| {
        x + y + z
    }).collect();

    let mut sky1: Vec<i32> = sky;
    // let mut sky1: Vec<i32> = v;
    //for_each 遍历不返回

    let _ = sky1.iter().for_each(|x| {
        println!("for_each = {}", x);
    });
    //sky1.iter_mut 迭代修改
    for item in &mut sky1.iter_mut() {
        // item = item +1;
        println!("item ={:#?}", item);
    }
}

#[allow(dead_code)]
fn test_5() {
    let mut elements = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let compressed_round_constants = [0];
    // let constants_offset = 3;

    for (element, round_constant) in elements.iter_mut().zip(
        compressed_round_constants
            .iter()
        // .skip(constants_offset),
    ) {
        println!("element = {}", element);
        println!("round_constant = {}", round_constant);
        //
        //  *element =  *element + *round_constant;
        element.add_assign(*round_constant);
        //   sky
        // element.add_assign(round_constant);
    }

    for item in elements.iter() {
        // item = item +1;
        println!("item ={:#?}", item);
    }
}


#[allow(dead_code)]
fn test_6() {
    let mut a = 2;
    let b = 3;
    a.add_assign(b);
    a.mul_assign(b);

    println!("a = {}", a);

    // let mut elements = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut compressed_round_constants = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let constants_offset = 3;
    //
    // for (element, round_constant) in elements.iter_mut().zip(
    //     compressed_round_constants
    //         .iter()
    //         .skip(constants_offset),
    // ) {
    //     println!("element = {}",element);
    //     println!("round_constant = {}",element);
    //     //
    //     //  *element =  *element + *round_constant;
    //     element.add_assign(*round_constant);
    //     //   sky
    //     // element.add_assign(round_constant);
    // }
    //
    // for item in elements.iter() {
    //     // item = item +1;
    //     println!("item ={:#?}", item);
    // }
}


#[allow(dead_code)]
fn test_7() {
    // let v: Vec<i32> = Vec::new();
    let v: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let sky = v.iter().enumerate().take(6).map(|(x, y)| {
        x + y
    });
    let items1: Vec<_> = sky.collect();
    println!("wwww  = {:#?}", items1);
}

#[allow(dead_code)]
fn test_8() {
    // let v: Vec<i32> = Vec::new();
    let nums = [1, 2, 3, 4, 5];
    let result2 = nums.iter().fold(4, |acc, &x| {
        acc * x
    });
    println!("{:?}", result2);
}

#[allow(dead_code)]
fn test_9() {
    // let v: Vec<i32> = Vec::new();
    // let nums = [1, 2, 3, 4, 5];
    let result2 = &[1, 2, 3, 4, 5].iter().map(|v| {
        let vvv = 1 * (v);
        vvv
    });
    println!("{:?}", result2);
}


fn test_10() {
    let sky = [[1u8;10];10];
    let sky1 = sky.iter().as_slice();


   let kk = &sky[..][..];

    // test_11(&[1,2,3,4]);
}

fn test_11(a: &[u8]) {
    let sky = a.to_vec();
    let arr = &sky[..];
    println!("sky = {:?}", sky);
    println!("arr = {:?}", arr);
}



fn test_12() {
    let mut s = [0 as u8; 12];
    // s[0] = b'h';
    // s[1] = b'e';
    // s[2] = b'l';
    // s[3] = b'l';
    // s[4] = b'o';

    s[0] = 11;
    s[1] = 33;
    s[2] = 44;
    s[3] = 55;
    s[4] = 66;

    let mut s1 = [0 as u8; 12];
    // s[0] = b'h';
    // s[1] = b'e';
    // s[2] = b'l';
    // s[3] = b'l';
    // s[4] = b'o';

    s[0] = 11;
    s[1] = 33;
    s[2] = 44;
    s[3] = 55;
    s[4] = 66;

    // let s = vec![1,2,3,4];
    // let s1 = vec![1,2,3];
    // let s = s.as_slice();
    // let s1 = s1.as_slice();
    //

    // let s = &[1,2,3]
    // let mut s_1:&str = str::from_utf8(&[1,2,3]).unwrap();
    // s_1 = str::from_utf8(&[1,2,3,4]).unwrap();
    // println!("kkkk = {:?}", s_1);
    //
    // let s1_1:&str = str::from_utf8(&[1,2,3,40]).unwrap();
    // println!("www = {:?}", s1_1);
    // let s_1 = &[1,2,3,6];
    // let s1_1 = &[1,2,3,4];
    if   &s[..] == &s1[..] {
        println!("相等")
    }else {
        println!("不相等")
    }

}


// pub struct Hugo{
//     #[warn(dead_code)]
//     aa:usize,
//     bb:usize
// }
