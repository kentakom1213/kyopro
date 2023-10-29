//         C - Linear Approximation        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc102/tasks/arc100_a
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
        A: [isize; N],
    }

    let A = A
        .iter()
        .zip(0..N as isize)
        .map(|(&v, i)| v - i)
        .sorted()
        .collect_vec();

    let med = A[(N - 1) / 2];

    let ans = A.iter().map(|&v| (v - med).abs()).sum::<isize>();

    println!("{}", ans);
}
