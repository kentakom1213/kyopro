// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

const FIVE: [usize; 5] = [0, 2, 4, 6, 8];

fn main() {
    input! {
        mut N: usize,
    }

    if N == 1 {
        println!("0");
        return;
    }

    N -= 1;

    // 5進数を考える
    let mut tmp = vec![];

    while N > 0 {
        tmp.push(FIVE[N % 5]);
        N /= 5;
    }

    println!("{}", tmp.iter().rev().join(""));
}
