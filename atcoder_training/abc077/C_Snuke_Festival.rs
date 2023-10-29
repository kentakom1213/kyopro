//            C - Snuke Festival
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc077/tasks/arc084_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        mut ABC: [[usize; N]; 3],
    }

    for i in 0..3 {
        ABC[i].sort();
    }

    // DPテーブル
    let mut dp = vec![vec![1; N]; 3];

    // 更新
    for i in 0..2 {
        // 直前のテーブルの累積和
        let S = accumulationSum(&dp[i]);
        // 全てのjについて更新
        for j in 0..N {
            let max_idx = ABC[i].upper_bound(&(ABC[i + 1][j] - 1));
            dp[i + 1][j] = S(0, max_idx);
        }
    }

    let ans = dp[2].iter().sum::<usize>();
    println!("{}", ans);
}

/// ## accumulationSum
fn accumulationSum(array: &Vec<usize>) -> impl Fn(usize, usize) -> usize {
    let mut sum = vec![0; array.len() + 1];
    for i in 0..array.len() {
        sum[i + 1] = sum[i] + array[i];
    }
    move |l: usize, r: usize| {
        sum[r] - sum[l]
    }
}
