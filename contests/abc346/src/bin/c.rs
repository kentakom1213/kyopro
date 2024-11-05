#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let ng = A
        .iter()
        .sorted()
        .dedup()
        .take_while(|&&v| v <= K)
        .sum::<usize>();

    let total = K * (K + 1) / 2;

    let ans = total - ng;

    println!("{ans}");
}
