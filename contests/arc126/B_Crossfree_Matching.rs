//         B - Cross-free Matching         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc126/tasks/arc126_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        mut L: [(usize, usize); M],
    }

    // ソート
    L.sort_by_key(|&(a, b)| (a, Reverse(b)));
    debug!(&L);

    // bを基準にLISを求める
    let mut dp = vec![INF; M + 1];
    for &(a, b) in &L {
        let idx = dp.lower_bound(&b);
        dp[idx] = b;
        debug!(&dp);
    }

    let ans = dp.lower_bound(&INF);

    println!("{}", ans);
}