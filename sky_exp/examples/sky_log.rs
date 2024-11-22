use std::any::type_name_of_val;

#[allow(dead_code)]
fn main() {
    // test_1();
    let a = vec![1u32;4];
    println!("a = {:?}", a);
    println!("type_name_of_val = {}",  type_name_of_val(&a)) ;
    println!("size_of_val = {}", std::mem::size_of_val(&a)) ;
    let b: &[u8] = unchecked_cast(&a);


    println!("b = {:?}", b);

    let c: &[u32] = unchecked_cast(&b);
    println!("c = {:?}", c);
    fn unchecked_cast<A, B>(a: &[A]) -> &[B] {
        let new_len = std::mem::size_of_val(a) / std::mem::size_of::<B>();
        unsafe { std::slice::from_raw_parts(a.as_ptr() as *const B, new_len) }
    }

}


//无类型
fn test_1() {
   let height = get_merkle_tree_height(64, 2);
   println!("test_1 height= {:?}",height)
}

pub fn get_merkle_tree_height(leafs: usize, branches: usize) -> usize {
    (branches as f64 * leafs as f64).log(branches as f64) as usize
}