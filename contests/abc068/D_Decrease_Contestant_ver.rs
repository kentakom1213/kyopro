//      D - Decrease (Contestant ver.)     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc068/tasks/arc079_b
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
use proconio::{input, fastout, marker::{Chars, Usize1}};

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

const N: usize = 50;

fn main() {
    input! {
        K: usize,
    }

    let mut arr = vec![];
    
    for _ in 0..K % N {
        arr.push(N);
    }

    for _ in 0..N - K % N {
        arr.push(N - 1 - K % N);
    }

    debug!(&arr);

    for i in 0..N {
        arr[i] += K / N;
    }

    println!("{}", N);
    println!("{}", arr.iter().join(" "));
}
