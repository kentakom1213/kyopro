// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        S: String,
        X: String,
    }

    let S = S
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    // dp[i] := 
    //     X[i] == 'A' => 
    //     X[i] == 'T' => 
}
