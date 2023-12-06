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

fn main() {
    input! {
        N: usize,
        points: [(f64, f64); N]
    }

    // dp[i][j] := チェックポイントiをj番目に通過するときのコストの最小値
    let mut dp = vec![vec![INF; N + 1]; N + 1];

    for i in 0..N {
        for j in 0..N {
            
        }
    }
}

const INF: f64 = 1e20;
