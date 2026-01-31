#![allow(non_snake_case)]

use cp_library_rs::debug2D;
use proconio::input;

const MAX: usize = 20;

fn main() {
    input! {
        N: usize,
    }

    // i 桁で末尾が d の N で割ったあまりが p であるような良い整数が存在するか．
    let mut dp = vec![vec![vec![false; N]; 10]; MAX];

    for d in 1..=9 {
        dp[0][d][d % N] = true;
    }

    debug2D!(dp[0]);

    for i in 1..MAX {
        for p in 0..N {
            for c in 0..=9 {
                if !dp[i - 1][c][p] {
                    continue;
                }
                for d in 0..=c {
                    let nxt = (p * 10 + d) % N;
                    dp[i][d][nxt] = true;
                }
            }
        }

        debug2D!(dp[i + 1]);

        for d in 0..=9 {
            if dp[i][d][0] {
                println!("OK: {}, {}", i, d);
                return;
            }
        }
    }
}
