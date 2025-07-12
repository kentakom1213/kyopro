#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        CL: [(char, usize); N]
    }

    if CL
        .iter()
        .map(|&(_, n)| n)
        .fold(0_usize, |acc, n| acc.saturating_add(n))
        > 100
    {
        println!("Too Long");
        return;
    }

    let ans = CL
        .into_iter()
        .map(|(c, n)| c.to_string().repeat(n))
        .join("");

    println!("{ans}");
}
