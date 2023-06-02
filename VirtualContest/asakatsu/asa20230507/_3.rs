// https://atcoder.jp/contests/abc180/tasks/abc180_d

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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        mut X: u128,
        Y: u128,
        A: u128,
        B: u128,
    }

    let mut exp = 0;

    // 倍にして増やす
    while X * A < X + B && X * A < Y {
        exp += 1;
        X *= A;
    }

    debug!(exp, Y-X, B);

    // 加算する
    exp += (Y - X - 1) / B;

    println!("{}", exp);
}