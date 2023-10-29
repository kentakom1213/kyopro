//                A - Make M               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc161/tasks/arc161_a
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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort();
    let mid = N / 2 + 1;

    // 前半と後半をzip
    let mut is_ok = true;

    for i in 0..N / 2 {
        debug!(A[i], A[i + mid], A[i + 1]);
        is_ok &= A[i] < A[i + mid] && A[i + mid] > A[i + 1];
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
