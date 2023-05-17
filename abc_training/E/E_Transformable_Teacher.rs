//        E - Transformable Teacher        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc181/tasks/abc181_e
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
        N: usize,
        M: usize,
        H: [usize; N],
        W: [usize; M],
    }

    // sum[i] := 左からi//2番目までをペアにしていったときの差の合計
    let mut sum = vec![0; N + 1];
    for i in 0..N {
        if N % 2 == 0 {
            sum[i + 1] = sum[i];
        }
    }
}
