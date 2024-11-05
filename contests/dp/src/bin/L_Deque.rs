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
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - MiniMax法
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    let mut dp = vec![vec![None; N]; N];

    let ans = f(0, N - 1, N, &A, &mut dp);

    println!("{}", ans);
}

/// 先頭i, 末尾jのときのスコア
fn f(i: usize, j: usize, N: usize, A: &Vec<isize>, memo: &mut Vec<Vec<Option<isize>>>) -> isize {
    debug!(i, j);
    if let Some(val) = memo[i][j] {
        return val;
    }

    let res = if i == j {
        if N % 2 == 0 {
            -A[i]
        } else {
            A[i]
        }
    } else {
        if (i + j) % 2 != N % 2 {
            // 太郎の手番
            let mut tmp = -INF;
            if i + 1 < N {
                chmax!(tmp, f(i + 1, j, N, A, memo) + A[i]);
            }
            if j > 0 {
                chmax!(tmp, f(i, j - 1, N, A, memo) + A[j]);
            }
            tmp
        } else {
            // 次郎の手番
            let mut tmp = INF;
            if i + 1 < N {
                chmin!(tmp, f(i + 1, j, N, A, memo) - A[i]);
            }
            if j > 0 {
                chmin!(tmp, f(i, j - 1, N, A, memo) - A[j]);
            }
            tmp
        }
    };

    memo[i][j] = Some(res);
    res
}
