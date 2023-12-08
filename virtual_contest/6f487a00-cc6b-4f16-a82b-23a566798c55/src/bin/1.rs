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
        H: usize,
        W: usize,
        mut S: [Chars; H]
    }

    // 貪欲に置き換える
    for r in 0..H {
        for c in 0..W - 1 {
            if S[r][c] == 'T' && S[r][c + 1] == 'T' {
                S[r][c] = 'P';
                S[r][c + 1] = 'C';
            }
        }
    }

    for row in S {
        println!("{}", row.iter().join(""));
    }
}
