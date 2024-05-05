//     F - Sensor Optimization Dilemma
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc325/tasks/abc325_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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

fn main() {
    input! {
        N: usize,
        D: [usize; N],
        (L1, C1, K1): (usize, usize, usize),
        (L2, C2, K2): (usize, usize, usize),
    }

    // dp[i][j] := i番目の区間までみて，1種類目のセンサーをj個使うときのセンサー2の個数の最小値
    let mut dp = vec![vec![INF; K1 + 1]; N + 1];

    // 初期化
    for j in 0..=K1 {
        dp[0][j] = 0;
    }

    for i in 1..=N {
        for j in 0..=K1 {
            for k in 0..=j {
                // D[i]に対してk本使うとき
                let rem = (D[i - 1].saturating_sub(k * L1) + L2 - 1) / L2;
                if dp[i - 1][j - k] + rem <= K2 {
                    chmin! {
                        dp[i][j],
                        dp[i - 1][j - k] + rem
                    };
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        for r in &dp {
            println!("{:?}", r);
        }
    }

    // コストの最小値を計算
    let mut ans = INF;
    for j in 0..=K1 {
        chmin! {
            ans,
            C1 * j + C2 * dp[N][j]
        };
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
