//         E - Sequence Decomposing        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc134/tasks/abc134_e
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
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
    }

    A.reverse();

    // LIS
    let mut dp = vec![INF; N];

    for &a in &A {
        let idx = dp.upper_bound(&a);
        dp[idx] = a;
    }

    debug!(&dp);

    let ans = dp.iter().take_while(|&&v| v < INF).count();
    println!("{}", ans);
}
