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

    // dp[i][r] :=
    //     'A' => {r | (10*r%7) in dp[i+1] or ((10*r + S[i])%7) in dp[i]}
    //     'T' => {r | (10*r%7) in dp[i+1] and ((10*r + S[i])%7) in dp[i]}
    let mut dp = vec![0_u8; N + 1];
    dp[N] = 1;

    // 後ろから埋めていく
    for (i, (d, x)) in (0..N).rev().zip(S.chars().rev().zip(X.chars().rev())) {
        let d = d.to_digit(10).unwrap();
        // ビットごとに考える
        for r in 0..7 {
            if x == 'A' {
                dp[i] |= (dp[i + 1] >> (10 * r % 7) & 1 & dp[i + 1] >> ((10 * r + d) % 7) & 1) << r;
            }
            if x == 'T' {
                dp[i] |= (dp[i + 1] >> (10 * r % 7) & 1 | dp[i + 1] >> ((10 * r + d) % 7) & 1) << r;
            }
        }
    }

    if cfg!(debug_assertions) {
        for &x in &dp {
            eprintln!("{:0>7b}", x)
        }
    }

    if dp[0] & 1 == 1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
