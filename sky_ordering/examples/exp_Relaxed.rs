use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;

/// 上面 assert 可能会失败，也就是 Z 的值在最后可能为 1。
/// 在函数 `write_x_then_y` 里面即使 store X happens-before store Y，即使在 `read_y_then_x` 里面 load Y 返回了 true，X 的值仍然可能是 false。
/// 因为对 X 和 Y 的两个操作都是 relaxed 的，虽然对于不同的线程，两个 load 或者两个 store 都可能满足 happens-before，
/// 但在 store 和 load 之间，并没有相关的约束，也就是意味着 load 可能看到乱序的 store。
/// 通常来说，relaxed 适用的场景就是需要对某个变量进行原子操作，而且不需要考虑多个线程同步的情况，譬如 reference counter，其它时候需要考虑有更强约束的其他 ordering。

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);
static Z: AtomicI32 = AtomicI32::new(0);

fn write_x_then_y() {
    X.store(true, Ordering::Relaxed);
    Y.store(true, Ordering::Relaxed);
}

fn read_y_then_x() {
    while !Y.load(Ordering::Relaxed) {}
    if X.load(Ordering::Relaxed) {
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        write_x_then_y();
    });

    let t2 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    assert_ne!(Z.load(Ordering::SeqCst), 0);
}