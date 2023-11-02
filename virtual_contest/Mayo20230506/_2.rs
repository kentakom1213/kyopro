// https://atcoder.jp/contests/abc185/tasks/abc185_c

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const SIZ: usize = 300;


// main
fn main() {
    input! {
        L: usize,
    }

    // 組合せの列挙
    let mut dp = vec![vec![0_usize; SIZ + 1]; SIZ + 1];
    for i in 0..SIZ {
        dp[i][0] = 1;
        dp[0][i] = 1;
    }

    for i in 0..SIZ {
        for j in 0..SIZ {
            dp[i + 1][j + 1] = dp[i + 1][j].saturating_add( dp[i][j + 1] );
        }
    }

    let comb = |n: usize, r: usize| -> usize {
        dp[n - r][r]
    };

    {
        #![cfg(debug_assertions)]
        for r in &dp[..20] {
            println!("{:?}", &r[..20])
        }
    }

    let ans = comb(L + 12 - 1 - 12, 11);
    println!("{}", ans);
}