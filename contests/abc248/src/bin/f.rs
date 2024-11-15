#![allow(non_snake_case)]

use cp_library_rs::{debug2D, number_theory::dynamic_modint::Modint2};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: usize,
    }

    // dp[i][j][k] := ブロックiまで見て，j個の辺を切ったとき，
    //     k = 0 → ブロックi+1は自由に切って良い
    //     k = 1 → ブロックi+1で上下を切ってはいけない
    let mut dp = vec![vec![[Modint2::new(0, P); 2]; N]; N];

    dp[0][0][0] += 1;
    dp[0][1][1] += 1;

    for i in 0..N - 1 {
        for j in 0..N {
            let prv0 = dp[i][j][0];
            let prv1 = dp[i][j][1];

            // 切らない場合
            dp[i + 1][j][0] += prv0 + prv1;

            if j + 1 <= N - 1 {
                // 上，下，右のいずれか1つを切る
                dp[i + 1][j + 1][0] += prv0 * 3;

                // 切れてる状態から右を切る
                dp[i + 1][j + 1][1] += prv1;
            }

            // 上+右，下+右の組合せで切る
            if j + 2 <= N - 1 {
                dp[i + 1][j + 2][1] += prv0 * 2;
            }
        }
    }

    debug2D!(dp);

    println!("{}", (1..=N - 1).map(|i| dp[N - 1][i][0]).join(" "));
}
