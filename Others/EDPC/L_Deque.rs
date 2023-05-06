//                L - Deque                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_l
// ----------------------------------------

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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - MiniMax法
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // 太郎の手番であるか判定する
    let isTaro = |i: usize, j: usize| -> bool {
        (j - i) % 2 == (N - 1) % 2
    };

    // dp[i][j] := 先頭がA[i]、末尾がA[j]のとき、X-Yの最大値/最小値
    let mut dp = vec![vec![0; N]; N];


}


