#![allow(non_snake_case)]

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N]
    }

    let mut set = FxHashSet::default();

    for i in 0..N - M + 1 {
        for j in 0..N - M + 1 {
            let mut rect = String::new();
            for r in i..i + M {
                rect += &S[r][j..j + M];
            }
            set.insert(rect);
        }
    }

    println!("{}", set.len());
}
