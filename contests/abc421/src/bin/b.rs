#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        X: usize,
        Y: usize
    }

    let mut A = vec![0; 11];
    A[1] = X;
    A[2] = Y;

    for i in 3..=10 {
        A[i] = f(A[i - 1] + A[i - 2]);
    }

    debug!(A);

    println!("{}", A[10]);
}

fn f(mut s: usize) -> usize {
    let mut res = 0;
    while s > 0 {
        res *= 10;
        res += s % 10;
        s /= 10;
    }
    res
}
