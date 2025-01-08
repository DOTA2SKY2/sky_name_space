#[allow(dead_code)]
fn main() {
    test_6()
}


/// 字符串转数字，需指定数字类型(如：usize)
fn test_1() {
    let guess: usize = "42".parse().expect("Not a number!");
}


/// ** 整型溢出
/// debug 模式:编译时，Rust 会检查整型溢出， panic
/// release 模式构建时，Rust 不检测溢出， 补码循环溢出 （two’s complement wrapping）




/// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理
fn test_2() {
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
}

/// 如果使用 checked_* 方法时发生溢出，则返回 None 值
fn test_3() {
    let a : u8 = 255;
    let b = a.checked_add(20).unwrap();
    println!("{}", b);  // 19
}



/// 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
fn test_4() {
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
}

/// 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值
fn test_5() {
    let a : u8 = u8::MAX;
    let b = a.overflowing_add(1);
    println!("val = {:?}, overflowing={:?}", b.0, b.1);  // 19
}


/// HashMap 数据结构 K 的类型必须实现了 std::cmp::Eq 特征, 整数类型、字符串类型、布尔类型都实现了该特征, 作为 HashMap 的 Key.

/// NaN 对于数学上未定义的结果
fn test_6() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}