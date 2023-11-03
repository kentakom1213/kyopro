// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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
        M: usize,
        edges: [(Usize1, Usize1, usize); M]
    }

    let mut dp = vec![vec![INF; N]; N];

    // 値をセット
    for &(u, v, w) in &edges {
        dp[u][v] = w;
    }

    for i in 0..N {
        dp[i][i] = 0;
    }

    let mut ans = 0;

    // フロイド・ワーシャル法
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin! {
                    dp[i][j],
                    dp[i][k] + dp[k][j]
                };

                if dp[i][j] < INF {
                    ans += dp[i][j];
                }

                if cfg!(debug_assertions) {
                    eprintln!("\nk:{k}, i:{i}, j:{j} -> {ans}");
                    for row in &dp {
                        eprintln!("{:?}", row);
                    }
                }
            }
        }
    }

    println!("{ans}");
}
