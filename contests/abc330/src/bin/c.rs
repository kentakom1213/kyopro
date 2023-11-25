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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        D: i128,
    }

    let mut ans = INF as i128;

    let cost = |x: i128, y: i128| -> i128 { (x * x + y * y - D).abs() };

    // x*x <= Dの範囲で全探索
    // x <= yに限定
    let mut x = 0;
    while x * x <= D {
        // yを二分探索
        let mut left = x - 1;
        let mut right = D + 10;
        while right - left > 2 {
            let m1 = (left * 2 + right) / 3;
            let m2 = (left + right * 2) / 3;
            if cost(x, m1) > cost(x, m2) {
                left = m1;
            } else {
                right = m2;
            }
        }

        for y in left..=right {
            debug!(x, y, cost(x, y));
            chmin! {
                ans,
                cost(x, y)
            };
        }

        x += 1;
    }

    println!("{}", ans);
}
