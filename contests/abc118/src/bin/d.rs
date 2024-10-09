#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, debug2D, utils::consts::INF};
use proconio::input;

const MATCH: [usize; 10] = [INF, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; M]
    }

    A.sort();

    // 作るのに必要なマッチの本数（使えない場合はINF）
    let B: Vec<usize> = A.iter().map(|&a| MATCH[a]).collect();

    // 部分和DP
    // dp[i][j] := i文字目までをちょうどj本の文字を使ってつくるとき，文字数の最大値
    let mut dp = vec![vec![-1; N + 1]; M + 1];

    dp[0][0] = 0;

    for i in 0..M {
        for j in 0..=N {
            // 使わないとき
            if dp[i][j] >= 0 {
                chmax! {
                    dp[i + 1][j],
                    dp[i][j],
                };
            }

            // 使うとき
            if j + B[i] <= N {
                if dp[i][j] >= 0 {
                    chmax! {
                        dp[i + 1][j + B[i]],
                        dp[i][j] + 1,
                    };
                }
                if dp[i + 1][j] >= 0 {
                    chmax! {
                        dp[i + 1][j + B[i]],
                        dp[i + 1][j] + 1,
                    };
                }
            }
        }
    }

    debug2D!(dp);

    // 復元（大きい文字から貪欲に）
    let mut ans = String::new();
    let mut j = N;

    for i in (0..M).rev() {
        while j >= B[i] && dp[i + 1][j - B[i]] + 1 == dp[i + 1][j] {
            debug!(i, j);
            j -= B[i];
            ans += &A[i].to_string();
        }
    }

    println!("{ans}");
}
