#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    println!(
        "{}",
        A.iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| a * b)
            .join(" ")
    );
}
