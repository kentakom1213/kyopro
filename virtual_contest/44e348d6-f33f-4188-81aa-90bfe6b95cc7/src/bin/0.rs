// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        S: String
    }

    let mut tmp = 0;

    for (i, c) in S.chars().enumerate() {
        match c {
            'A' => {
                tmp |= 1;
            }
            'B' => {
                tmp |= 2;
            }
            'C' => {
                tmp |= 4;
            }
            _ => (),
        }
        if tmp == 7 {
            println!("{}", i + 1);
            return;
        }
    }
}
