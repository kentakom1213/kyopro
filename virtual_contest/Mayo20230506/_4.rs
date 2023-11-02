// https://atcoder.jp/contests/abc163/tasks/abc163_d

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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let sum = |a: usize, b: usize| -> usize {
        (a + b) * (b - a + 1) / 2
    };

    let mut ans = 1;

    // 10^100の組合せ * 1の組合せ
    for i in K..=N {
        let min = sum(0, i - 1);
        let max = sum(N - i + 1, N);
        ans += max - min + 1;
        ans %= MOD;
        debug!(i, min, max, max - min + 1);
    }

    println!("{}", ans);
}