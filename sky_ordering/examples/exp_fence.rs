use std::sync::atomic::{AtomicBool, AtomicI32, fence, Ordering};
use std::thread;

/// 在上面的例子中，2 Release fence 是 synchronizes-with 5 Acquire fence 的，
/// 而4 load Y 的时候一定会读取到 3 store Y 的值，
/// 加上 1 store X 是 sequenced-before 3 的，
/// 那么自然能确定 1 是 happens-before 6 的。
/// 也就是 Z 一定不会等于 0。

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);
static Z: AtomicI32 = AtomicI32::new(0);


fn write_x_then_y() {
    X.store(true, Ordering::Relaxed); // 1
    fence(Ordering::Release);         // 2
    Y.store(true, Ordering::Relaxed); // 3
}

fn read_y_then_x() {
    while !Y.load(Ordering::Relaxed) {}  // 4
    fence(Ordering::Acquire);            // 5
    if X.load(Ordering::Relaxed) {       // 6
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