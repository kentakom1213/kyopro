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
        S: Chars,
        K: Usize1,
    }

    // Kまですべて1
    if (0..K).all(|i| S[i] == '1') {
        println!("{}", S[K]);
    }
    // 初めて出てくる1以外の文字
    else {
        for &c in S.iter() {
            if c != '1' {
                println!("{}", c);
                return;
            }
        }
    }
}
