use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;


/// 不同于之前的 relaxed，这里我们对 Y 使用了 Acquire 和 Release，那么最后 Z 就一定不可能为 0 了。
/// 主要是因为 store Y 是 synchronizes-with load Y 的，也就是 store Y happens before load Y，
/// 因为 store X 是 sequenced-before store Y，那么 store X 就是 happens-before load X 的。
/// 通常，我们还可以使用 AcqRel ordering，它其实就是组合了 Acquire 和 Release，对于 load 使用 Acquire，而对于 store 则是使用 Release。

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);
static Z: AtomicI32 = AtomicI32::new(0);

fn write_x_then_y() {
    X.store(true, Ordering::Relaxed);
    Y.store(true, Ordering::Release);
}

fn read_y_then_x() {
    while !Y.load(Ordering::Acquire) {}
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