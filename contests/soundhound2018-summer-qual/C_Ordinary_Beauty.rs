//           C - Ordinary Beauty           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_c
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
        n: f64,
        m: f64,
        d: f64,
    }

    if d == 0.0 {
        // 隣り合う要素が同じ数字になる組合せ
        let ans = (m - 1.0) / n;
        println!("{}", ans);
    }
    else {
        let ans = (m - 1.0) * 2.0 * (n - d) / (n * n);
        println!("{:.9}", ans);
    }
}
