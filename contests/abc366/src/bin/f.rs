#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, number_theory::frac::Frac};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        mut AB: [(isize, isize); N]
    }

    AB.sort_by_key(|&(a, b)| Frac(a - 1, b));

    // dp[j] := i番目までからj個選んだときの最大値
    let mut dp = [0; 11];
    dp[0] = 1;

    for &(a, b) in &AB {
        let mut ndp = dp;
        for j in 0..K {
            chmax! {
                ndp[j + 1],
                dp[j] * a + b
            };
        }
        dp = ndp;
        debug!(dp);
    }

    println!("{}", dp[K]);
}
