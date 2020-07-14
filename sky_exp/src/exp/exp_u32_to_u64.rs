#[allow(dead_code)]
pub fn main_u32_to_u64() {
    test();
}

//高位转低位
#[allow(dead_code)]
fn test(){
    let a:u32 = 0x55;
    let mut buffer = [0u8; 32];
    let sky = a as u64;
    buffer[..8].copy_from_slice(&sky.to_be_bytes());
    println!("jjj = {:?}",buffer);
}

#[allow(dead_code)]
fn test1(){
    let a:u32 = 0x55;
    let mut buffer = [0u8; 32];
    let sky = a as u64;
    buffer[..8].copy_from_slice(&sky.to_be_bytes());
    println!("jjj = {:?}",buffer);
}
