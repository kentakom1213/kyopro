//         D - Semi Common Multiple
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc150/tasks/abc150_d
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }

    let c = A.iter().map(|&v| v / 2).fold(0, |a, b| gcd(a, b));

    // cの奇数倍がXまでに何個存在するか
    // c * (2k + 1) <= X
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, b % a)
    }
}
