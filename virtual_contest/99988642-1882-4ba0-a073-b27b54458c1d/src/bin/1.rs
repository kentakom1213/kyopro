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
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        P: usize,
        A: [usize; N],
        B: [usize; M],
    }

    // Bをソート
    let B = B.iter().cloned().sorted().rev().collect_vec();

    let mut ans = 0;

    // Aを全探索
    for &a in &A {
        let thr = B.upper_bound(&P.saturating_sub(a));
        
    }

    println!("{}", ans);
}
