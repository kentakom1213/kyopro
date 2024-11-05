#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    print!("{} ", A[N - K..].iter().join(" "));
    println!("{}", A[..N - K].iter().join(" "));
}
