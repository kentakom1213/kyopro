


// https://atcoder.jp/contests/dp/tasks/dp_f

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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// ## LCS
/// 最長共通部分列を得る
/// 計算量：O(NM)
fn LCS<T: std::cmp::PartialEq> (A: &[T], B: &[T]) -> usize {
    let (la, lb) = (A.len(), B.len());
    let mut dp = vec![vec![0; lb+1]; la+1];

    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            if a == b {
                chmax!(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax!(dp[i+1][j+1], dp[i+1][j]);
            chmax!(dp[i+1][j+1], dp[i][j+1]);
        }
    }

    dp[la][lb]
}

/// ## LCS with Vector
/// 最長共通部分列を得る
/// 計算量：O(NM)
fn LCS_with_Vec<T: std::cmp::PartialEq + Copy> (A: &[T], B: &[T]) -> Vec<T> {
    let (la, lb) = (A.len(), B.len());
    let mut dp = vec![vec![0; lb+1]; la+1];

    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            if a == b {
                chmax!(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax!(dp[i+1][j+1], dp[i+1][j]);
            chmax!(dp[i+1][j+1], dp[i][j+1]);
        }
    }

    let mut res: Vec<T> = vec![];
    let (mut r, mut c) = (la, lb);
    while r > 0 && c > 0 && res.len() < dp[la][lb] {
        if dp[r][c] == dp[r-1][c] {
            r -= 1;
        } else if dp[r][c] == dp[r][c-1] {
            c -= 1;
        } else {
            res.push(A[r-1]);
            r -= 1;
            c -= 1;
        }
    }

    {
        #![cfg(debug_assertions)]
        for r in &dp {
            println!("{:?}", r);
        }
    }

    res.reverse();
    res
}

// main
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let lcs = LCS_with_Vec(&S, &T);

    println!("{}", lcs.iter().join(""));
}
