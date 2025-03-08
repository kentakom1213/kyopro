#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        D: usize,
        A: [usize; N]
    }

    // i個選んで総和%Dがjのときの最大値
    let mut dp = vec![vec![None; D]; K + 1];

    dp[0][0] = Some(0);

    for &a in &A {
        let mut nxt = dp.clone();

        for i in 0..K {
            for j in 0..D {
                if let Some(x) = dp[i][j] {
                    chmax!(nxt[i + 1][(j + a) % D], Some(x + a));
                }
            }
        }

        dp = nxt;
    }

    if cfg!(debug_assertions) {
        for i in 0..=K {
            debug!(i, dp[i]);
        }
    }

    // Dの倍数を探索
    if let Some(ans) = dp[K][0] {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
