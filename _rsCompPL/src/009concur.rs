use rayon::prelude::*;
use std::time::{Duration, Instant};

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fibonacci_join(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let (a, b) = rayon::join(|| fib_recursive(n - 1), || fib_recursive(n - 2));
    a + b
}

fn main() {
    let start = Instant::now();
    let x = fib_recursive(48);
    let duration = start.elapsed();
    println!("Recursive fib -> {}, time : {:?}", x, duration);

    println!("Now Run With Rayon's Join.");

    let start = Instant::now();
    let x = fibonacci_join(48);
    let duration = start.elapsed();
    println!("Rayon fib -> {}, time : {:?}", x, duration);
}
