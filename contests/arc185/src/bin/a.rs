#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        M: usize,
    }

    let rem = N * (N + 1) % M;

    if 1 <= rem && rem <= N {
        println!("Bob");
    } else {
        println!("Alice");
    }
}
