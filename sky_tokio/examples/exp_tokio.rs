mod exp_dir;

pub(crate) use tokio;

/// 单前线程 #[tokio::main(flavor = "current_thread")]
/// 多线程 #[tokio::main(flavor = "multi_thread", worker_threads = 1)]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    exp1().await
}

async fn exp1() {
    println!("hello tokio")
}