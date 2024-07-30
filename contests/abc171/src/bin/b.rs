#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N]
    }

    let ans = P.iter().sorted().take(K).sum::<usize>();

    println!("{ans}");
}
