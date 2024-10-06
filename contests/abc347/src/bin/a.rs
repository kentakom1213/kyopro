#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    println!(
        "{}",
        A.iter().filter(|&&x| x % K == 0).map(|x| x / K).join(" ")
    );
}
