#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        S: String,
        X: Chars,
    }

    let S = S
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

}
