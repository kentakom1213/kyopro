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

fn main() {
    input! {
        N: usize,
        L: isize,
        R: isize,
        A: [isize; N]
    }

    let isok = |a: isize, x: isize| {
        (x - a).abs() <= (L - a).abs() && (x - a).abs() <= (R - a).abs()
    };

    let ans = A
        .iter()
        .map(|&a| {
            if a <= L {
                L
            } else if a >= R {
                R
            } else {
                a
            }
        })
        .collect_vec();

    println!("{}", ans.iter().join(" "));
}
