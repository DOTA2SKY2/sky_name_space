use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::thread;

/// 上面的例子，只有使用 SeqCst ordering，才能保证 Z 最后的值不为 0，任何其他的 ordering，都不能保证，
/// 我们来具体分析一下。因为两个 read 函数都是有 while 循环，退出之前一定能确保 write 函数被调用了。
/// 因为使用 SeqCst 能保证所有线程看到一致的操作顺序，假设 3 返回了 false，表明 X 为 true，而 Y 为 false，这时候一定能保证 store Y 还没调用，一定能保证 store X 在 store Y 之前发生，4 就一定会返回 true。
/// 如果这里我们对 load 使用 Acquire，而对 store 使用 Release，`read_x_then_y` 和 `read_y_then_x` 可能看到完全相反的对 X 和 Y 的操作顺序。
/// SeqCst 在有些时候，可能会有性能瓶颈，因为它需要确保操作在所有线程之前全局同步，但是它其实又是最直观的一种使用方式， 所以通常，当我们不知道用什么 ordering 的时候，用 SeqCst 就对了。

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);
static Z: AtomicI32 = AtomicI32::new(0);

fn write_x() {
    X.store(true, Ordering::SeqCst);    // 1
}

fn write_y() {
    Y.store(true, Ordering::SeqCst);    // 2
}

fn read_x_then_y() {
    while !X.load(Ordering::SeqCst) {}
    if Y.load(Ordering::SeqCst) {       // 3
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn read_y_then_x() {
    while !Y.load(Ordering::SeqCst) {}
    if X.load(Ordering::SeqCst) {       // 4
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        write_x();
    });

    let t2 = thread::spawn(move || {
        write_y();
    });

    let t3 = thread::spawn(move || {
        read_x_then_y();
    });

    let t4 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

    assert_ne!(Z.load(Ordering::SeqCst), 0);
}