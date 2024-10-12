#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        mut A: [usize; N]
    }

    A.insert(K, X);

    println!("{}", A.iter().join(" "));
}
