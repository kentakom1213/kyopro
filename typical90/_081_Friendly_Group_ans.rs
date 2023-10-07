//         081 - Friendly Group（★5）        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_cc
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use num_traits::Num;

pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![T::zero(); W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i][j + 1] + S[i + 1][j] - S[i][j] + array[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
        S[r_end][c_end]
            - S[r_end][c_start]
            - S[r_start][c_end]
            + S[r_start][c_start]
    }
}

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(usize, usize); N],
    }

    let SIZE = 5050;

    // マッピング
    let mut arr = vec![vec![0; SIZE]; SIZE];
    for &(a, b) in &AB {
        arr[a][b] += 1;
    }

    // 2次元累積和を求める
    let acc = acc2D(&arr);

    // KxKの領域を全探索
    let mut ans = 0;
    for i in 0..SIZE - K - 1 {
        for j in 0..SIZE - K - 1 {
            chmax! {
                ans,
                acc(i, i + K + 1, j, j + K + 1)
            };
        }
    }

    println!("{ans}");
}
