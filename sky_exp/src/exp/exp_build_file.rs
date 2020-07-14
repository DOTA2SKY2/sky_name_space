static GIT_VERSION_FILE: &str = include_str!("git_version_file");
pub fn main_build_file(){
        println!("GIT_VERSION_FILE = {:?}",GIT_VERSION_FILE);
}