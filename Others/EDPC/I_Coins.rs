//                I - Coins
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_i
// ----------------------------------------

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        P: [f64; N],
    }

    // dp[i][j] := i枚目までのコインを投げたとき、表がj枚出る確率
    let mut dp = vec![vec![0.0; N + 1]; N + 1];
    dp[0][0] = 1.0;

    for i in 0..N {
        for j in 0..N {
            // 表が出る
            dp[i + 1][j + 1] += dp[i][j] * P[i];
            // 裏が出る
            dp[i + 1][j] += dp[i][j] * (1.0 - P[i]);
        }
    }

    debug_2d(&dp);

    let ans = dp[N][N / 2 + 1..].iter().sum::<f64>();
    println!("{}", ans);
}
