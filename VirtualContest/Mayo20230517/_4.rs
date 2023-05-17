// https://atcoder.jp/contests/abc133/tasks/abc133_d

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
        A: [isize; N],
    }

    let mut B1 = 0;
    for i in 0..N {
        if i % 2 == 0 {
            B1 += A[i];
        } else {
            B1 -= A[i];
        }
    }

    print!("{} ", B1);

    // 残りの答えを求めていく
    let mut B = B1 / 2;
    for i in 0..N - 1 {
        B = A[i] - B;
        print!("{} ", B * 2);
    }
    println!();
}
