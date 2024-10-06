#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug2D};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: Chars
    }

    // dp[i][j] := i回目に[R,P,S]を出したとき、勝つ回数の最大値
    let mut dp = vec![[0; 3]; N + 1];

    for i in 0..N {
        // // R
        // chmax! {
        //     dp[i + 1][0],
        //     dp[i][1],
        //     dp[i][2],
        // };
        // // P
        // chmax! {
        //     dp[i + 1][1],
        //     dp[i][0],
        //     dp[i][2],
        // };
        // // S
        // chmax! {
        //     dp[i + 1][2],
        //     dp[i][0],
        //     dp[i][1],
        // };
        
        match S[i] {
            'R' => {
                // R
                chmax! {
                    dp[i + 1][0],
                    dp[i][1],
                    dp[i][2],
                };
                // P
                chmax! {
                    dp[i + 1][1],
                    dp[i][0],
                    dp[i][2],
                };
                dp[i + 1][1] += 1;
            }
            'P' => {
                // P
                chmax! {
                    dp[i + 1][1],
                    dp[i][0],
                    dp[i][2],
                };
                // S
                chmax! {
                    dp[i + 1][2],
                    dp[i][0],
                    dp[i][1],
                };
                dp[i + 1][2] += 1;
            }
            'S' => {
                // R
                chmax! {
                    dp[i + 1][0],
                    dp[i][1],
                    dp[i][2],
                };
                // S
                chmax! {
                    dp[i + 1][2],
                    dp[i][0],
                    dp[i][1],
                };
                dp[i + 1][0] += 1;
            }
            _ => ()
        }
    }

    debug2D!(dp);

    let ans = dp[N].iter().max().unwrap();

    println!("{ans}");
}

