#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let B = A.iter().cloned().zip(1..).sorted().collect_vec();

    let ans = B[N - 2].1;

    println!("{ans}");
}

