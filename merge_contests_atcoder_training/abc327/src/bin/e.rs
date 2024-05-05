// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::Roots;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
const FMINF: f64 = -(INF as f64);

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}
/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
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

fn main() {
    input! {
        N: usize,
        mut P: [usize; N]
    }

    // Pを逆順に
    P.reverse();

    // dp[i][j] := コンテストiに後ろからj回目として出る時の最大値
    let mut dp = vec![vec![FMINF; N + 1]; N + 1];
    
    for i in 0..N {
        dp[i][0] = 0.0;
    }

    for i in 1..=N {
        let mut d = 1.0;
        for j in 1..=N {
            // 選ばないとき
            chmax! {
                dp[i][j],
                dp[i - 1][j],
            };
            // j回目として選ぶとき
            chmax! {
                dp[i][j],
                dp[i - 1][j - 1] + d * P[i - 1] as f64
            };
            d *= 0.9;
        }
    }

    if cfg!(debug_assertions) {
        for row in &dp {
            eprintln!("{:?}", row);
        }
    }

    // 1回以上参加する時の最大値を求める
    let mut ans = FMINF;

    for i in 1..=N {
        let mut sum = 1.0;
        let mut d = 1.0;
        for j in 1..=N {
            chmax! {
                ans,
                dp[i][j] / sum - 1200.0 / (j as f64).sqrt()
            };
            d *= 0.9;
            sum += d;
        }
    }

    println!("{}", ans);
}
