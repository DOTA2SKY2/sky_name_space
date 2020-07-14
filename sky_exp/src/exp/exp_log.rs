#[allow(dead_code)]
pub fn main_log() {
    test_1();

}


//无类型
fn test_1() {
   let height = get_merkle_tree_height(64, 2);
   println!("test_1 height= {:?}",height)
}

pub fn get_merkle_tree_height(leafs: usize, branches: usize) -> usize {
    (branches as f64 * leafs as f64).log(branches as f64) as usize
}