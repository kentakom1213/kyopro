// https://atcoder.jp/contests/abc239/tasks/abc239_d

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

/// 素数判定
fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n < i*i { break; }
        if n % i == 0 {
            return false;
        }
    }
    true
}

// main
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize
    }

    let mut isAoki = true;

    for x in A..=B {
        let mut any = false;
        for y in C..=D {
            debug!(x + y, is_prime(x + y));
            any |= is_prime(x + y);
        }
        isAoki &= any;
    }

    if !isAoki {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}