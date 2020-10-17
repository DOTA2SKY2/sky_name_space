use std::{io, fs, mem};
use core::num;
use std::net::{IpAddr, Ipv6Addr, Ipv4Addr};

/** From关键字
*
pub trait From<T> {
    fn from(T) -> Self;
}
用于在使用输入值时进行值到值的转换。它是Into的倒数。
人们应该总是喜欢从上到下实现，因为由于标准库中的全面实现，从上到下的实现会自动提供到的实现。
只有在需要转换到当前板条箱之外的类型时，才将其实现为。由于Rust的孤立规则，From不能进行此类转换。查看更多细节。
在泛型函数上指定特征界限时，喜欢使用Into而不是From。这样，直接实现为的类型也可以用作参数。
在执行错误处理时，From也非常有用。当构造一个可能失败的函数时，返回类型通常是Result<T, E>。通过允许函数返回封装多种错误类型的单一错误类型，From trait简化了错误处理。有关更多细节，请参阅“示例”部分和该书。
注意:这个特性一定不能失败。如果转换失败，请使用TryFrom。
**/

pub fn main_from() {
    test_box_string();
}

/**
*   字符串
**/
#[allow(dead_code)]
fn test_string() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
}

/**
*   file
在执行错误处理时，为自己的错误类型实现From通常很有用。通过将底层错误类型转换为封装底层错误类型的自定义错误类型，我们可以返回单个错误类型，
而不会丢失底层原因的信息。“?'操作符通过调用<CliError>::将底层错误类型自动转换为我们的自定义错误类型，
将其转换为实现From时Into<CliError>::into。然后编译器推断应该使用Into的哪个实现。
**/
#[allow(dead_code)]
fn test_file() {
    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(error: io::Error) -> Self {
            CliError::IoError(error)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(error: num::ParseIntError) -> Self {
            CliError::ParseError(error)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        let contents = fs::read_to_string(&file_name)?;
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }

    let res = open_and_parse_file("ss");
    match res {
        Ok(x) => {
            println!("x = {}", x);
        }
        Err(w) => {
            match w {
                CliError::IoError(xx) => {
                    println!("CliError::IoError = {}", xx);
                }
                CliError::ParseError(xx) => {
                    println!("CliError::ParseError = {}", xx);
                }
            }
        }
    }
}

/**
*   Ip6
**/
#[allow(dead_code)]
fn test_ip6addr1() {
    let addr = IpAddr::from([525u16, 524u16, 523u16, 522u16, 521u16, 520u16, 519u16, 518u16, ]);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0x20d, 0x20c, 0x20b, 0x20a, 0x209, 0x208, 0x207, 0x206)), addr);

    let addr = IpAddr::from([25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8, 17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8, ]);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0x1918, 0x1716, 0x1514, 0x1312, 0x1110, 0x0f0e, 0x0d0c, 0x0b0a)), addr);

    let addr = Ipv6Addr::from(0x102030405060708090A0B0C0D0E0F00D_u128);
    assert_eq!(
        Ipv6Addr::new(
            0x1020, 0x3040, 0x5060, 0x7080,
            0x90A0, 0xB0C0, 0xD0E0, 0xF00D,
        ),
        addr);
}


/**
*   Ip4
**/
#[allow(dead_code)]
fn test_ip4addr1() {
    let addr = IpAddr::from([13u8, 12u8, 11u8, 10u8]);
    assert_eq!(IpAddr::V4(Ipv4Addr::new(13, 12, 11, 10)), addr);

    let addr = Ipv4Addr::from([13u8, 12u8, 11u8, 10u8]);
    assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);

    let addr = Ipv4Addr::from(0x0d0c0b0au32);
    assert_eq!(Ipv4Addr::new(13, 12, 11, 10), addr);
}


/**
*  impl From<bool> for i128
**/
#[allow(dead_code)]
fn test_i128() {
    assert_eq!(i128::from(true), 1);
    assert_eq!(i128::from(false), 0);
}

/**
*  fn from(i: u8) -> char
**/
#[allow(dead_code)]
fn test_char() {
    let u = 32 as u8;
    let c = char::from(u);
    assert_eq!(4, mem::size_of_val(&c));
}


/** 字符串转字符串数组
* fn from(s: Box<str>) -> Box<[u8]>
**/
#[allow(dead_code)]
fn test_box() {
    // create a Box<str> which will be used to create a Box<[u8]>
    let boxed: Box<str> = Box::from("hello");
    let boxed_str: Box<[u8]> = Box::from(boxed);

// create a &[u8] which will be used to create a Box<[u8]>
    let slice: &[u8] = &[104, 101, 108, 108, 111];
    let boxed_slice = Box::from(slice);

    assert_eq!(boxed_slice, boxed_str);
}


/** 字符串与字符串数组互相转换
* fn from(s: Box<str>) -> Box<[u8]>
**/
fn test_box_string() {
    let s1: String = String::from("hello world");
    let s2: Box<str> = s1.into_boxed_str();
    let s3: String = String::from(s2);
    assert_eq!("hello world", s3);
}


fn test_from_struct(){
    struct Squre{
        with:i32,
        height:i32
    }
}





