#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug2D, utils::consts::INF};
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        H: usize,
        W: usize,
        R: [usize; H],
        C: [usize; W],
        A: [String; H],
    }

    let A: Vec<Vec<usize>> = A
        .into_iter()
        .map(|row| row.chars().map(|x| (x == '1') as usize).collect())
        .collect();

    // dp[i][j][r][c] := マス(i,j)に到達するルートのうち，
    //     | r == 0 && c == 0 : 行i,列j ともそのまま
    //     | r == 1 && c == 0 : 行i のみ反転
    //     | r == 0 && c == 1 : 列j のみ反転
    //     | r == 1 && c == 1 : 行i,列j どちらも反転
    //    するようなもののコストの最小値
    let mut dp = vec![vec![[[INF; 2]; 2]; W]; H];

    dp[0][0][0][0] = 0;
    dp[0][0][1][0] = R[0];
    dp[0][0][0][1] = C[0];
    dp[0][0][1][1] = R[0] + C[0];

    for i in 0..H {
        for j in 0..W {
            // 左から
            if j > 0 {
                for (r, c, pc) in iproduct!(0..2, 0..2, 0..2) {
                    if A[i][j - 1] ^ pc == A[i][j] ^ c {
                        chmin! {
                            dp[i][j][r][c],
                            dp[i][j - 1][r][pc] + [0, C[j]][c]
                        };
                    }
                }
            }

            // 上から
            if i > 0 {
                for (r, c, pr) in iproduct!(0..2, 0..2, 0..2) {
                    if A[i - 1][j] ^ pr == A[i][j] ^ r {
                        chmin! {
                            dp[i][j][r][c],
                            dp[i - 1][j][pr][c] + [0, R[i]][r]
                        };
                    }
                }
            }
        }
    }

    debug2D!(dp);

    let mut ans = INF;

    chmin! {
        ans,
        dp[H - 1][W - 1][0][0],
        dp[H - 1][W - 1][1][0],
        dp[H - 1][W - 1][0][1],
        dp[H - 1][W - 1][1][1],
    };

    println!("{ans}");
}
