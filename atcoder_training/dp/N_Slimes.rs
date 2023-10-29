//                N - Slimes
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_n
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

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 連鎖行列積の計算順序問題に似てる
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    let mut dp = vec![vec![None; N]; N];

    // 答えを求める
    rec(0, N - 1, &A, &S, &mut dp);

    debug_2d(&dp);

    println!("{}", dp[0][N - 1].unwrap());
}

/// 配列`arr`を適当に分割して順に合体させていくときの、コストの最小値
fn rec(
    i: usize,
    j: usize,
    A: &Vec<usize>,
    S: &Vec<usize>,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(res) = memo[i][j] {
        return res;
    }

    let res = if i == j {
        0
    } else {
        let mut res = INF;
        for k in i..j {
            let tmp = rec(i, k, A, S, memo) + rec(k + 1, j, A, S, memo);
            chmin!(res, tmp);
        }
        res + S[j + 1] - S[i]
    };

    memo[i][j] = Some(res);
    res
}
