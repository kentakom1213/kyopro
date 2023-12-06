//            B - The Middle Day           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc315/tasks/abc315_b
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
        M: usize,
        D: [usize; M],
    }

    // 合計
    let mid = (D.iter().sum::<usize>() + 1) / 2;

    let mut cur = 0;
    for i in 0..M {
        cur += D[i];
        if mid <= cur {
            println!("{} {}", i + 1, mid - (cur - D[i]));
            return;
        }
    }
}
