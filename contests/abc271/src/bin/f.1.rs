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
use rustc_hash::{FxHashMap, FxHashSet};

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

fn main() {
    input! {
        N: usize,
        A: [[usize; N]; N],
    }

    // // 組合せの調査
    // let mut dp = vec![vec![0_u128; N]; N];
    // for i in 0..N {
    //     dp[i][0] = 1;
    //     dp[0][i] = 1;
    // }
    // for i in 1..N {
    //     for j in 1..N {
    //         dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    //     }
    // }
    // debug2D!(dp);

    // 領域を(左上, 右下)の2つの対角領域に分割し，マージする

    let half = N / 2;

    // 左上から
    let mut left = vec![FxHashMap::default(); N];
    // (half, i) に到達するときの総xor
    dfs_left(0, 0, 0, &A, &mut left);

    debug!(left);

    // 右下から
    let mut right = vec![FxHashMap::default(); N];
    // (half, i) に到達するときの総xor
    dfs_right(N - 1, N - 1, 0, &A, &mut right);

    debug!(right);

    // マージする
    let mut ans = 0_usize;

    for (i, l) in left.iter().enumerate() {
        for (lk, lv) in l {
            if let Some(rv) = right[i].get(lk) {
                ans += lv * rv;
            }
        }
    }

    println!("{ans}");
}

fn dfs_left(
    r: usize,
    c: usize,
    xor: usize,
    A: &Vec<Vec<usize>>,
    left: &mut [FxHashMap<usize, usize>],
) {
    let N = A.len();
    if r + c == N - 1 {
        let tmp = xor ^ A[r][N - r - 1];
        *left[r].entry(tmp).or_insert(0) += 1;
    } else {
        // 右に移動
        if c + 1 < N {
            dfs_left(r, c + 1, xor ^ A[r][c], A, left);
        }
        // 下に移動
        if r + 1 < N {
            dfs_left(r + 1, c, xor ^ A[r][c], A, left);
        }
    }
}

fn dfs_right(
    r: usize,
    c: usize,
    xor: usize,
    A: &Vec<Vec<usize>>,
    right: &mut [FxHashMap<usize, usize>],
) {
    let N = A.len();
    if r + c == N - 1 {
        *right[r].entry(xor).or_insert(0) += 1;
    } else {
        // 左に移動
        if c > 0 {
            dfs_right(r, c - 1, xor ^ A[r][c], A, right);
        }
        // 上に移動
        if r > 0 {
            dfs_right(r - 1, c, xor ^ A[r][c], A, right);
        }
    }
}
