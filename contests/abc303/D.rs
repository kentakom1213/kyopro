// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        S: String,
    }

    let N = S.len();

    // DP
    // dp[i][j] := i文字目でCapsLockが(j?on:off)のとき、i文字目までを押すのにかかる時間の最小値
    let mut dp = vec![vec![INF; 2]; N + 1];
    dp[0][0] = 0;

    for (i, c) in S.chars().enumerate() {
        debug!(i, c);
        if c == 'a' {
            // 押さない
            chmin!(dp[i + 1][0], dp[i][0] + X);
            chmin!(dp[i + 1][1], dp[i][1] + Y);
            // 押す
            chmin!(dp[i + 1][1], dp[i][0] + Z + Y);
            chmin!(dp[i + 1][0], dp[i][1] + Z + X);
        } else {
            // 押さない
            chmin!(dp[i + 1][0], dp[i][0] + Y);
            chmin!(dp[i + 1][1], dp[i][1] + X);
            // 押す
            chmin!(dp[i + 1][1], dp[i][0] + Z + X);
            chmin!(dp[i + 1][0], dp[i][1] + Z + Y);
        }
    }

    debug_2d(&dp);

    let ans = dp[N][0].min(dp[N][1]);
    println!("{}", ans);
}
