// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
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
        A: [isize; N],
    }

    let MINF = -(INF as isize);

    // dp[i][j] := A[i]をj番目に選ぶときの最大値
    let mut dp = vec![vec![MINF; M + 1]; N + 1];
    
    for i in 0..N {
        dp[i][0] = 0;
    }

    for i in 1..=N {
        for j in 1..=M {
            // 選ばない
            chmax! {
                dp[i][j],
                dp[i - 1][j]
            };
            // 選ぶ
            if dp[i - 1][j - 1] != MINF {
                chmax! {
                    dp[i][j],
                    dp[i - 1][j - 1] + A[i - 1] * j as isize
                };
            }
        }
    }

    if cfg!(debug_assertions) {
        for r in &dp {
            eprintln!("{:?}", r);
        }
    }
    
    println!("{}", dp[N][M]);
}
