//             C - Blue Spring             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc318/tasks/abc318_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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
        N: usize,
        D: usize,
        P: usize,
        mut F: [usize; N]
    }

    // Fをソート
    F.sort();

    // 累積和を取る
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + F[i];
    }

    // パスをsセット購入
    let mut ans = INF;
    for s in 0..=(N + D - 1) / D {
        let mut cost = P * s;
        // パスで賄えない分を直接購入
        if s * D <= N {
            cost += S[N - s * D];
        }
        debug!(s, cost);
        ans = ans.min(cost);
    }

    println!("{}", ans);
}
