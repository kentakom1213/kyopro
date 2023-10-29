//           D -  Three Days Ago
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc295/tasks/abc295_d
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        S: String,
    }
    // 数字の列に直す
    let S = S.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    let N = S.len();

    // 個数をmod2でカウント（パターンは2^10通りなので整数で管理する）
    let mut dp = vec![0; N + 1];
    for i in 0..N {
        dp[i + 1] = dp[i] ^ (1 << S[i]);
    }

    {
        #![cfg(debug_assertions)]
        for &v in &dp {
            println!("{:0>10b}", v);
        }
    }

    // パターンの個数をカウント
    let mut cnt = vec![0_usize; 1 << 10];
    for &v in &dp {
        cnt[v] += 1;
    }

    debug!(&cnt);

    // 組合せを計算
    let mut ans = 0;
    for &v in &cnt {
        ans += v * v.saturating_sub(1) / 2;
    }

    println!("{}", ans);
}
