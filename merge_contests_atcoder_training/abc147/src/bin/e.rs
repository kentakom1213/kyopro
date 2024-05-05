#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[isize; W]; H],
        B: [[isize; W]; H],
    }

    // C[i][j] := A[i][j] - B[i][j]
    let C = A
        .iter()
        .zip(&B)
        .map(|(a, b)| a.iter().zip(b).map(|(x, y)| x - y).collect_vec())
        .collect_vec();

    debug2D!(C);

    // dp[i][j][k] := (i,j)への経路のうち，(C[r][c],-C[r][c])をうまく選んで足し合わせることで，
    //                その総和をkにすることができるか．
    let mut dp = vec![vec![vec![false; MAX as usize]; W]; H];

    dp[0][0][(OFFSET - C[0][0]) as usize] = true;
    dp[0][0][(OFFSET + C[0][0]) as usize] = true;

    for i in 0..H {
        for j in 0..W {
            for k in 0..MAX {
                // 上から
                if i > 0 && dp[i - 1][j][k as usize] {
                    let pos = k + C[i][j];
                    if 0 <= pos && pos < MAX {
                        dp[i][j][pos as usize] = true;
                    }
                    let neg = k - C[i][j];
                    if 0 <= neg && neg < MAX {
                        dp[i][j][neg as usize] = true;
                    }
                }
                // 左から
                if j > 0 && dp[i][j - 1][k as usize] {
                    let pos = k + C[i][j];
                    if 0 <= pos && pos < MAX {
                        dp[i][j][pos as usize] = true;
                    }
                    let neg = k - C[i][j];
                    if 0 <= neg && neg < MAX {
                        dp[i][j][neg as usize] = true;
                    }
                }
            }
        }
    }

    // 最小値を見つける
    let ans = (OFFSET - MAX..)
        .zip(&dp[H - 1][W - 1])
        .filter_map(|(x, &isok)| isok.then_some(x.abs()))
        .min()
        .unwrap();

    println!("{ans}");
}

const OFFSET: isize = 6500;
const MAX: isize = OFFSET * 2;
const INF: usize = 1001001001001001001;
