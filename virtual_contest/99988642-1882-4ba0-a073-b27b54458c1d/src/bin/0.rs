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
        N: usize,
        X: usize,
        AB: [(usize, usize); N],
    }

    let mut ans = INF;
    let mut sum = 0;

    for (i, &(a, b)) in AB.iter().enumerate() {
        sum += a;
        chmin! {
            ans,
            sum + (X - i) * b
        };
        sum += b;
    }

    println!("{}", ans);
}
