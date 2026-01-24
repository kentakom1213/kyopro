#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, number_theory::modint::M107, utils::num_traits::Zero};
use proconio::input;

const MOD: usize = 13;

fn main() {
    input! {
        S: String
    }
    let N = S.len();

    // dp[i][j] := S[..i] まで見たとき，13 で割ったあまりが j になる置き換えの数
    let mut dp = vec![[M107::zero(); MOD]; N + 1];
    dp[0][0] += 1;

    for (i, c) in S.chars().enumerate() {
        match c {
            '0'..='9' => {
                let x = c as usize - '0' as usize;

                for u in 0..MOD {
                    let tmp = dp[i][u];
                    dp[i + 1][(u * 10 + x) % 13] += tmp;
                }
            }
            _ => {
                for x in 0..=9 {
                    for u in 0..MOD {
                        let tmp = dp[i][u];
                        dp[i + 1][(u * 10 + x) % 13] += tmp;
                    }
                }
            }
        }
    }

    debug2D!(dp);

    let ans = dp[N][5];

    println!("{ans}");
}
