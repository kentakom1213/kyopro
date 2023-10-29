//                A - Apple                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc265/tasks/abc265_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
        X: usize,
        Y: usize,
        N: usize,
    }

    let ans = if X * 3 >= Y {
        Y * (N / 3) + X * (N % 3)
    } else {
        X * N
    };

    println!("{}", ans);
}
