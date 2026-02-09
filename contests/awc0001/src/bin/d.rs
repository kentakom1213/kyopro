#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug2D};
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        AB: [(i64, usize); N]
    }

    // dp[s][j] := 滞在費の合計が s 円で，町 j の宿に最後に滞在したときの利益の最大値
    let mut dp = vec![vec![-1; N]; M + 1];

    for i in 0..N {
        let (a, b) = AB[i];
        let mut ndp = vec![vec![-1; N]; M + 1];

        // 町 j の宿から始める
        ndp[b][i] = a;

        for s in 0..=M {
            for j in 0..i {
                // 移動しない
                chmax! {
                    ndp[s][j],
                    dp[s][j]
                };

                // j から i に移動する
                if i - j <= K && s + b <= M {
                    chmax! {
                        ndp[s + b][i],
                        dp[s][j] + a
                    };
                }
            }
        }

        dp = ndp;
    }

    debug2D!(dp);

    let mut ans = 0;

    for s in 0..=M {
        for i in 0..N {
            chmax!(ans, dp[s][i]);
        }
    }

    println!("{ans}");
}
