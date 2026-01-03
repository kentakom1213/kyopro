#![allow(non_snake_case)]

use cp_library_rs::debug2D;
use proconio::input;

fn main() {
    input! {
        N: String,
    }

    let K = N.len();

    // dp[i][d][f] := i桁目までみたとき，1がd回出現する数でN以下であることが確定して(f?いない:いる)数の個数
    let mut dp = vec![vec![[0; 2]; K + 2]; K + 1];

    dp[0][0][0] = 1;

    for (i, x) in N.chars().map(ord).enumerate() {
        for d in 0..=K {
            for y in 0..=9 {
                let is1 = (y == 1) as usize;
                // 未確定 → 確定
                if x > y {
                    dp[i + 1][d + is1][1] += dp[i][d][0];
                }
                // 未確定 → 未確定
                if x == y {
                    dp[i + 1][d + is1][0] += dp[i][d][0];
                }
                // 確定 → 確定
                dp[i + 1][d + is1][1] += dp[i][d][1];
            }
        }
    }

    debug2D!(dp);

    let ans = (1..=K)
        .map(|n| (dp[K][n][0] + dp[K][n][1]) * n)
        .sum::<usize>();

    println!("{ans}");
}

fn ord(c: char) -> usize {
    c as usize - '0' as usize
}
