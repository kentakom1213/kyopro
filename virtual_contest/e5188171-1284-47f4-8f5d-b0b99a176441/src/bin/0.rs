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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        X: usize,
        jp: [(usize, usize); N],
    }

    let mut dp = vec![vec![false; X + 1]; N + 1];

    dp[0][0] = true;

    for (i, &(a, b)) in jp.iter().enumerate() {
        for j in 0..=X {
            if dp[i][j] {
                if j + a <= X {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= X {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }

    // if cfg!(debug_assertions) {
    //     for row in &dp {
    //         println!("{:?}", row);
    //     }
    // }

    if dp[N][X] {
        println!("Yes");
    } else {
        println!("No");
    }
}
