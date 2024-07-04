#![allow(non_snake_case)]

use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize; N]
    }

    let mut dp = vec![false; 10100];
    dp[0] = true;
    let mut ndp = vec![false; 10100];

    for i in 0..N {
        for j in 0..=10000 {
            ndp[j] |= dp[j];
            if j >= P[i] {
                ndp[j] |= dp[j - P[i]];
            }
        }
        swap(&mut dp, &mut ndp);
    }

    let ans = dp.iter().filter(|x| **x).count();

    println!("{ans}");
}
