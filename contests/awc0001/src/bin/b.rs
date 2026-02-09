#![allow(non_snake_case)]

use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        R: usize,
        P: [usize; N]
    }

    let ans = P
        .into_iter()
        .enumerate()
        .filter(|&(_, x)| L <= x && x <= R)
        .max_by_key(|&(i, x)| (x, Reverse(i)));

    if let Some((i, _)) = ans {
        println!("{}", i + 1);
    } else {
        println!("-1");
    }
}
