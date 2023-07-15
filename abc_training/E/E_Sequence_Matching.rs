//          E - Sequence Matching          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc185/tasks/abc185_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    // dp[i][j] := A[..i]とB[..j]についての解
    let mut dp = vec![vec![INF; M + 1]; N + 1];
    dp[0][0] = 0;

    for (i, &a) in A.iter().enumerate() {
        dp[i + 1][0] = dp[i][0] + 1;
        for (j, &b) in B.iter().enumerate() {
            if i == 0 {
                dp[i][j + 1] = dp[i][j] + 1;
            }
            chmin!(
                dp[i + 1][j + 1],
                dp[i][j + 1] + 1,
                dp[i + 1][j] + 1,
                if a == b {
                    dp[i][j]
                } else {
                    dp[i][j] + 1
                }
            );
        }
    }

    if cfg!(debug_assertions) {
        for r in &dp {
            println!("{:?}", r);
        }
    }

    let ans = dp[N][M];
    println!("{}", ans);
}
