#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let acc = |s: &str| {
        let mut acc = vec![0_usize; N + 1];
        for (i, c) in s.chars().enumerate() {
            acc[i + 1] = acc[i] + (c == '#') as usize;
        }
        acc
    };

    let mut dp = vec![0; N + 1];

    for _ in 0..N {
        input! {
            s: String,
        }

        let sum = acc(&s);

        let mut ndp = vec![INF; N + 1];

        // min_(j <= k) dp[k]
        let mut min_k = INF;

        for j in (0..=N).rev() {
            // s[..j] を白く，s[j..] を黒く
            let to_white = sum[j];
            let to_black = (N - j) - (sum[N] - sum[j]);

            let cost = to_white + to_black;

            chmin!(min_k, dp[j]);
            chmin!(ndp[j], min_k + cost);
        }

        dp = ndp;
        debug!(dp);
    }

    let ans = dp.into_iter().min().unwrap();

    println!("{ans}");
}
