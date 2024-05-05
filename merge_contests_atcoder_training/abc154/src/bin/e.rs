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

pub fn comb(n: usize, r: usize) -> usize {
    if r == 0 {
        1
    } else if n < r {
        0
    } else {
        n * comb(n - 1, r - 1) / r
    }
}

fn main() {
    input! {
        S: String,
        K: usize,
    }

    let N: Vec<usize> = S
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let D = S.len();

    // 桁DP
    // dp0[i][j] := 上からi桁目まで決めて，0でない桁がj個あり，Nより小さいことが確定している
    let mut dp0 = vec![vec![0_usize; K + 2]; D + 1];

    // dp1[i][j] := 上からi桁目まで決めて，0でない桁がj個あり，Nより小さいことが確定していない
    let mut dp1 = vec![vec![0_usize; K + 2]; D + 1];

    dp1[0][0] = 1;

    for i in 0..D {
        for j in 0..=K {
            // 確定→確定（0を選ぶ）
            dp0[i + 1][j] += dp0[i][j];
            // 確定→確定（1以上を選ぶ）
            dp0[i + 1][j + 1] += dp0[i][j] * 9;

            // 未確定→確定
            if N[i] > 0 {
                dp0[i + 1][j + 1] += dp1[i][j] * (N[i] - 1);
                dp0[i + 1][j] += dp1[i][j];
            }

            // 未確定→未確定
            if N[i] == 0 {
                dp1[i + 1][j] = dp1[i][j];
            } else {
                dp1[i + 1][j + 1] = dp1[i][j];
            }
        }
    }

    if cfg!(debug_assertions) {
        eprintln!("dp0");
        for row in &dp0 {
            eprintln!("{:?}", row);
        }
        eprintln!("dp1");
        for row in &dp1 {
            eprintln!("{:?}", row);
        }
    }

    let ans = dp0[D][K] + dp1[D][K];
    println!("{}", ans);
}
