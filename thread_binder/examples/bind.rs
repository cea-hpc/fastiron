use rayon::prelude::*;
use thread_binder::ThreadPoolBuilder;

// let's have a slow computation so you can take time checking
// the binding with htop
pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .expect("Thread pool build failed");

    let fibo: Vec<_> = (0..46u64)
        .into_par_iter()
        .map(|i| fibonacci_recursive(i))
        .collect();
    assert_eq!(fibo[0], 0);
    println!("{:?}", fibo);
}
