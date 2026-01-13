#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        mut N: usize,
    }

    let mut set = FxHashSet::default();

    while !set.contains(&N) {
        set.insert(N);
        N = f(N);
        debug!(N, set);

        if N == 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn f(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|d| d as usize - '0' as usize)
        .map(|d| d * d)
        .sum::<usize>()
}
