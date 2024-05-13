// static GIT_VERSION_FILE: &str = include_str!("git_version_file");
use log::{info};
// #[macro_use]
// extern crate lazy_static;
struct ArsVersionInfo{}

impl ArsVersionInfo{
        pub fn version_init() -> ArsVersionInfo{
                info!("Ars Rust Version  名称:            {}", env!("CARGO_PKG_NAME"));
                info!("Ars Rust Version  版本号:           {}", env!("CARGO_PKG_VERSION"));
                info!("Ars Rust Version  仓库:            {}",env!("CARGO_PKG_REPOSITORY"));
                info!("Ars Rust Version  编译日期:         {}", env!("VERGEN_BUILD_DATE"));
                info!("Ars Rust Version  Git提交日期:      {}", env!("VERGEN_COMMIT_DATE"));
                info!("Ars Rust Version  Git提交号短:      {}", env!("VERGEN_SHA_SHORT"));
                info!("Ars Rust Version  Git提交号长:      {}", env!("VERGEN_SHA"));
                info!("Ars Rust Version  编译平台:         {}", env!("VERGEN_TARGET_TRIPLE"));
                info!("Ars Rust Version  作者:            {}", env!("CARGO_PKG_AUTHORS"));
                info!("Ars Rust Version  描述:            {}", env!("CARGO_PKG_DESCRIPTION"));
                ArsVersionInfo{}
        }
}

// static jjj:ArsVersionInfo = ArsVersionInfo::version_init();

lazy_static::lazy_static! {
    pub static ref KKK: i32 = version_init();
}

fn version_init() -> i32 {
        info!("Ars Rust Version  名称:            {}", env!("CARGO_PKG_NAME"));
        info!("Ars Rust Version  版本号:           {}", env!("CARGO_PKG_VERSION"));
        info!("Ars Rust Version  仓库:            {}",env!("CARGO_PKG_REPOSITORY"));
        info!("Ars Rust Version  编译日期:         {}", env!("VERGEN_BUILD_DATE"));
        info!("Ars Rust Version  Git提交日期:      {}", env!("VERGEN_COMMIT_DATE"));
        info!("Ars Rust Version  Git提交号短:      {}", env!("VERGEN_SHA_SHORT"));
        info!("Ars Rust Version  Git提交号长:      {}", env!("VERGEN_SHA"));
        info!("Ars Rust Version  编译平台:         {}", env!("VERGEN_TARGET_TRIPLE"));
        info!("Ars Rust Version  作者:            {}", env!("CARGO_PKG_AUTHORS"));
        info!("Ars Rust Version  描述:            {}", env!("CARGO_PKG_DESCRIPTION"));
        1
}
fn main(){
        let y = *KKK;
        // let y1 = *kkk;
        // let y = ArsVersionInfo::version_init();
}