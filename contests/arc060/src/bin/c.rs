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

fn main() {
    input! {
        N: usize,
        A: usize,
        X: [usize; N]
    }

    const MAX: usize = 55 * 55;

    // dp[i][j][k] := i枚目まで見てj枚選択しているとき，総和がkになる場合の数
    let mut dp = vec![vec![vec![0_usize; MAX]; 55]; 55];
    dp[0][0][0] = 1;

    for i in 0..N {
        let x = X[i];
        for j in 0..N {
            for k in 0..MAX {
                dp[i + 1][j][k] += dp[i][j][k];
                // 新しく追加
                if k + x < MAX {
                    dp[i + 1][j + 1][k + x] += dp[i][j][k];
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        for i in 0..5 {
            for j in 0..5 {
                eprintln!("{:?}", &dp[i][j][..40]);
            }
            eprintln!();
        }
    }

    // j枚選んだときの和がjAになっているものの数
    let ans = (1..=N).map(|j| dp[N][j][j * A]).sum::<usize>();

    println!("{ans}");
}
