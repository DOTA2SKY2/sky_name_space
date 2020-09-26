use std::ops::{AddAssign, MulAssign};
use std::time::SystemTime;
use std::{process, thread};
use log::info;

// use std::str;
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
    // test_13();
    // vec_sort_by();
    // vec_for1();
    // vec_for2();
    // vec_for3();
    // vec_for4();
    // vec_for5();
    vec_for8();
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
    let sky = [[1u8; 10]; 10];
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
    if &s[..] == &s1[..] {
        println!("相等")
    } else {
        println!("不相等")
    }
}

#[derive(Clone, Debug, Copy)]
struct SkyStruct {
    a: i32,
}

fn test_13() {
    let struct_1 = SkyStruct {
        a: 1
    };

    let struct_2 = SkyStruct {
        a: 2
    };

    let mut list: Vec<SkyStruct> = vec![struct_1, struct_2];
    // let sky = list[0];
    // sky.a = 100;

    let sky1 = list.get_mut(0).unwrap();
    sky1.a = 200;
    // *sky1.a = 200;
    list_test(&mut list);

    println!("{:?}", list.as_slice());

    let sky1 = list.get_mut(0).unwrap();
    println!("sky1 = {:?}", sky1);
    // for  in list {
    //
    // }
}

fn list_test(cs: &mut Vec<SkyStruct>) {
    let sky = cs.get_mut(0).unwrap();
    println!("{:?}", sky);
}


// pub struct Hugo{
//     #[warn(dead_code)]
//     aa:usize,
//     bb:usize
// }

fn slice_test() {
    let mut slice = [1, 2, 3, 4, 5];
    {
        let (left, right) = slice.split_at_mut(2);
        left.copy_from_slice(&right[1..]);
    }
    assert_eq!(slice, [4, 5, 3, 4, 5]);
}


fn slice_test1() {
    let mut slice = vec![1, 2, 3, 4, 5];
    {
        let mut start: Vec<i32> = vec![66, 55, 77777];
        let (left, right) = slice.split_at(2);
        println!("split_at left = {:?}", left);
        println!("split_at right = {:?}", right);

        let (left, right) = slice.split_at(2);
        let mut right = slice.split_off(2);
        // let mut  right = slice.(2);
        // println!("left = {:?}",left);
        println!("right = {:?}", right);
        // start.append(right.to_vec());
        // start.extend_from_slice(right);
        start.extend(right);
        println!("start = {:?}", start);
        // start.
        // start.copy_from_slice(right);
        // left.copy_from_slice(&right[1..]);
        // left.copy_from_slice(&right[1..]);
    }
    // println!()
    // assert_eq!(slice, [4, 5, 3, 4, 5]);
}

fn slice_test2() {
    let mut slice = vec![11, 12, 13, 14, 15];
    // for block in slice.chunks(10) {
    //     println!("chunks = {:?} ", block);
    // }

    // slice.split()
    //  slice.truncate(2);
    // println!("{:?}",slice);
    //  slice.truncate(3);
    // slice.reverse();
    for i in 0..slice.len() {
        println!("{:?}", slice.pop());
    }
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// #[derive(Debug,Ord)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age,
        }
    }
}


fn vec_sort_by() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 自然顺序，排序 people  (名字 和 年龄)
    people.sort();
    println!("1;;; = {:?}", people);
    // assert_eq!(
    //     people,
    //     vec![
    //         Person::new("Al".to_string(), 60),
    //         Person::new("John".to_string(), 1),
    //         Person::new("Zoe".to_string(), 25),
    //     ]);

    // 用 年龄 排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    println!("2;;; = {:?}", people);

    // assert_eq!(
    //     people,
    //     vec![
    //         Person::new("Al".to_string(), 60),
    //         Person::new("Zoe".to_string(), 25),
    //         Person::new("John".to_string(), 1),
    //     ]);
}


fn vec_for1() {
    let mut sky = vec![0; 2000 * 1000 * 1000];
    // sky.reserve(2000 * 1000 * 1000);
    let time = SystemTime::now();
    for x in 0..2000 * 1000 * 1000 {
        sky[x] = 1;
    }
    info!("sky[x]  用时 = {:?} s", time.elapsed().unwrap().as_secs());
}

fn vec_for2() {
    let mut sky = vec![];
    let time = SystemTime::now();
    for x in 0..2000 * 1000 * 1000 {
        sky.push(x);
    }
    info!("push 用时 = {:?} s", time.elapsed().unwrap().as_secs());
}

fn vec_for3() {
    let mut sky = vec![];
    let time = SystemTime::now();
    sky.reserve(2000 * 1000 * 1000);
    for x in 0..2000 * 1000 * 1000 {
        sky.push(x);
    }
    info!("push reserve 用时 = {:?} s", time.elapsed().unwrap().as_secs());
}

fn vec_for4() {
    let mut sky = Vec::with_capacity(2000 * 1000 * 1000);
    let time = SystemTime::now();
    sky.reserve(2000 * 1000 * 1000);
    for x in 0..2000 * 1000 * 1000 {
        sky.push(x);
    }
    info!("push with_capacity 用时 = {:?} s", time.elapsed().unwrap().as_secs());
}

fn vec_for5() {
    // let mut sky = Vec::with_capacity(2000 * 1000 * 1000);
    // let time = SystemTime::now();
    // sky.reserve(2000 * 1000 * 1000);
    // for x in 0.. 2000 * 1000 * 1000 {
    //     sky.push(x);
    // }
    // let time = SystemTime::now();

    // let num  = 2000 * 1000 * 1000;
    let num = 20;
    let sky = (0..num).into_iter().map(|e| {
        let i = e % 3;
        i
    }).collect::<Vec<_>>();
    // info!("i 用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    // let mut res = vec![];
    let mut res = Vec::with_capacity(num);
    // res.reserve(num);

    unsafe {
        res.set_len(200);
    }

    for (k, &v) in sky.iter().enumerate() {
        info!("with_capacity  k = {:?} ", k);
        res[k] = v;
    }

    unsafe {
        res.set_len(num);
    }
    info!("with_capacity  用时 = {:?} s", time.elapsed().unwrap().as_secs());


    let time = SystemTime::now();
    let mut res = vec![0; num];
    for (k, &v) in sky.iter().enumerate() {
        res[k] = v;
    }
    info!("res[k]  用时 = {:?} s", time.elapsed().unwrap().as_secs());


    let time = SystemTime::now();
    let mut res = vec![0; num];
    let mut res1 = vec![0; num];
    for &i in &sky {
        res.push(i);
        res1.push(i);
    }
    info!("push  用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    // let mut res1 = vec![0;2000 * 1000 * 1000];
    let res = sky.iter().map(|&x| {
        x
    }).collect::<Vec<_>>();
    info!(" map 用时 = {:?} s", time.elapsed().unwrap().as_secs());


    let time = SystemTime::now();
    let res1 = sky.iter().filter(|&&x| {
        true
    }).collect::<Vec<_>>();
    info!(" filter 用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    let res1 = sky.iter().filter_map(|&x| {
        Some(x)
    }).collect::<Vec<_>>();
    info!(" filter_map 用时 = {:?} s", time.elapsed().unwrap().as_secs());
}


fn vec_for6() {
    // let mut sky = Vec::with_capacity(2000 * 1000 * 1000);
    // let time = SystemTime::now();
    // sky.reserve(2000 * 1000 * 1000);
    // for x in 0.. 2000 * 1000 * 1000 {
    //     sky.push(x);
    // }
    // let time = SystemTime::now();

    // let num  = 2000 * 1000 * 1000;
    // let num  = 10;
    // let num  =  20;
    let sky = (0..10).map(|e| {
        e
    }).collect::<Vec<_>>();

    let sky1 = (0..20).map(|e| {
        e * 2
    }).collect::<Vec<_>>();
    // let ws= sky.chunks(10).map(|value|{
    //      println!("index = {:?} ; value = {:?}",value ,value);
    //     (value,value)
    //  }).collect::<Vec<_>>();
    let list = sky1.iter().zip(sky.iter()).map(|(v1, v2)| {
        println!("v1 = {:?} ; v2 = {:?}", v1, v2);
        (v1, v2)
    })
        .collect::<Vec<_>>();
}

fn vec_for7() {
    // let mut sky = Vec::with_capacity(2000 * 1000 * 1000);
    // let time = SystemTime::now();
    // sky.reserve(2000 * 1000 * 1000);
    // for x in 0.. 2000 * 1000 * 1000 {
    //     sky.push(x);
    // }
    // let time = SystemTime::now();
//------------------------------------
    let num  = 200 * 1000 * 1000;
    // let num = 20;
    let sky = (0..num).into_iter().map(|e| {
        let i = e % 3;
        i
    }).collect::<Vec<_>>();
    // info!("i 用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    // let mut res = vec![];
    let mut res = Vec::with_capacity(num);
    // res.reserve(num);

    unsafe {
        res.set_len(num);
    }
    let time = SystemTime::now();
    for (k, &v) in sky.iter().enumerate() {
        // info!("with_capacity  k = {:?} ", k);
        res[k] = v;
    }

    unsafe {
        res.set_len(num);
    }
    info!("with_capacity  用时 = {:?} s", time.elapsed().unwrap().as_secs());

//------------------------------------
    let num1  = 100 * 1000 * 1000;
    let num2  = 100 * 1000 * 1000;
    // let num = 20;
    let mut sky1 = (0..num1).into_iter().map(|e| {
        let i = e % 1;
        i
    }).collect::<Vec<_>>();

    let sky2 = (0..num2).into_iter().map(|e| {
        let i = e % 2;
        i
    }).collect::<Vec<_>>();
    let mut res = Vec::with_capacity(num1 + num2);
    // res.reserve(num);

    unsafe {
        res.set_len(num1 + num2);
    }
    let time = SystemTime::now();
    res[0..num1].copy_from_slice(&sky1[..]);
    res[num1..(num1+num2)].copy_from_slice(&sky2[..]);

    info!("copy_from_slice  用时 = {:?} s", time.elapsed().unwrap().as_secs());



    //------------------------------------
    let num1  = 100 * 1000 * 1000;
    let num2  = 100 * 1000 * 1000;
    // let num = 20;
    let mut sky1 = (0..num1).into_iter().map(|e| {
        let i = e % 1;
        i
    }).collect::<Vec<_>>();

    let mut sky2 = (0..num2).into_iter().map(|e| {
        let i = e % 2;
        i
    }).collect::<Vec<_>>();
    // let mut res = Vec::with_capacity(num1 + num2);
    // res.reserve(num);
    // res.reserve(num);

    // unsafe {
    //     res.set_len(num1 + num2);
    // }
    let mut res:Vec<i32> = Vec::new();
    let time = SystemTime::now();
    res.append(&mut sky1);
    res.append(&mut sky2);

    info!("copy_from_slice  用时 = {:?} s", time.elapsed().unwrap().as_secs());
}

fn vec_for8() {

    let num  = 200 * 1000 * 1000;
    let list = (0..num).into_iter().map(|e| {
        let i = e % 3;
        i
    }).collect::<Vec<_>>();

    let time = SystemTime::now();
    list.iter().for_each(|e| {

    });
    info!("for_each 用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    list.iter().for_each(|e| {

    });
    info!("for_each 用时 = {:?} s", time.elapsed().unwrap().as_secs());


    let time = SystemTime::now();
    let sky = list.iter().map(|&e| {
        let i = e % 3;
        i
    }).collect::<Vec<_>>();
    info!("map 用时 = {:?} s", time.elapsed().unwrap().as_secs());

    let time = SystemTime::now();
    // let mut res = vec![];
    let mut res = Vec::with_capacity(num);
    // res.reserve(num);

    unsafe {
        res.set_len(num);
    }
    let time = SystemTime::now();
    for (k, &v) in list.iter().enumerate() {
        // info!("with_capacity  k = {:?} ", k);
        res[k] = v;
    }

    unsafe {
        res.set_len(num);
    }
    info!("with_capacity  用时 = {:?} s", time.elapsed().unwrap().as_secs());

}

