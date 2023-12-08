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
    }

    let mut dp = vec![vec![0_usize; 50]; 50];

    for i in 0..=40 {
        dp[i][0] = 1;
        dp[0][i] = 1;
    }

    for i in 0..31 {
        for j in 0..31 {
            dp[i + 1][j + 1] = dp[i][j + 1].saturating_add(dp[i + 1][j]);
        }
    }

    // if cfg!(debug_assertions) {
    //     for row in &dp {
    //         eprintln!("{:?}", row);
    //     }
    // }

    // 表示
    for i in 1..=N {
        for j in 0..i {
            // debug!(i, j);
            print!("{} ", dp[i - j - 1][j]);
        }
        println!();
    }
}
