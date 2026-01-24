#![allow(non_snake_case)]

use cp_library_rs::{chmin, data_structure::acc2d::acc2D, debug, debug2D, utils::consts::INF};
use proconio::{input, marker::Chars};

/// 観察:
/// - (1,1) が白の場合，操作しなくてよい
/// - (N,N) が黒の場合，操作しなくてよい
/// - (N,N) を黒に塗らなくてよいのは，マスがすべて白の場合のみ
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let F: Vec<Vec<_>> = S
        .iter()
        .map(|row| row.iter().map(|c| (*c == '#') as usize).collect())
        .collect();

    let acc = acc2D(&F);

    // (白く塗るコスト, 黒く塗るコスト)
    let mut dp = vec![vec![(INF, INF); N]; N];

    for r in 0..N {
        for c in 0..N {
            let iswhite = (S[r][c] == '.') as usize;
            let isblack = (S[r][c] == '#') as usize;

            match (r > 0, c > 0) {
                (false, false) => {
                    dp[r][c] = (isblack, iswhite);
                }
                (false, true) => {
                    // 白 → 白
                    chmin!(dp[r][c].0, dp[r][c - 1].0 + isblack);
                    // 黒 → 黒
                    chmin!(dp[r][c].1, dp[r][c - 1].1 + iswhite);
                    // 白 → 黒
                    chmin!(dp[r][c].1, dp[r][c - 1].0 + iswhite);
                }
                (true, false) => {
                    // 白 → 白
                    chmin!(dp[r][c].0, dp[r - 1][c].0 + isblack);
                    // 黒 → 黒
                    chmin!(dp[r][c].1, dp[r - 1][c].1 + iswhite);
                    // 白 → 黒
                    chmin!(dp[r][c].1, dp[r - 1][c].0 + iswhite);
                }
                (true, true) => {
                    // (白,白) → 白
                    dp[r][c].0 = dp[r - 1][c].0 + dp[r][c - 1].0 + isblack;

                    debug!(dp[r - 1][c], dp[r][c - 1], dp[r - 1][c - 1]);

                    // → 黒
                    chmin!(
                        dp[r][c].1,
                        // 黒,黒
                        dp[r - 1][c].1 + dp[r][c - 1].1 + iswhite,
                        // 白,黒
                        dp[r - 1][c].0 + dp[r][c - 1].1 + iswhite,
                        // 黒,白
                        dp[r - 1][c].1 + dp[r][c - 1].0 + iswhite,
                    );

                    debug!(dp[r][c]);
                }
            }
        }
    }

    debug2D!(dp);

    let (w, b) = dp[N - 1][N - 1];
    let ans = w.min(b);

    println!("{}", ans);
}
