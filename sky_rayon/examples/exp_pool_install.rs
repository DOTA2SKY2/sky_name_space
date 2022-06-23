use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::current;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use rayon::ThreadPoolBuilder;

fn main() {
    // We only use a single thread to make the short-circuit behavior deterministic.
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let a = vec![0.0; 1024];
    let mut b = a.clone();
    b[42] = f64::NAN;

    pool.install(|| {
        let expected = None;
        assert_eq!(a.par_iter().partial_cmp(&b), expected);

        for len in 1..10 {
            let counter = AtomicUsize::new(0);
            let result = a
                .par_iter()
                .with_max_len(len)
                .inspect(|_| {
                    println!("{:?}",current().id());
                    counter.fetch_add(1, Ordering::SeqCst);
                })
                .partial_cmp(&b);
            assert_eq!(result, expected);
            // should not have visited every single one
            assert!(counter.into_inner() < a.len());
        }
    });
}