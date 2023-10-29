//              D - Christmas
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc115/tasks/abc115_d
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const MAX_SIZ: usize = 51;

// main
fn main() {
    input! {
        N: usize,
        X: usize,
    }

    let mut len = vec![0; MAX_SIZ]; // レベルiの段数
    let mut p = vec![0; MAX_SIZ]; // レベルiのパティの枚数
    len[0] = 1;
    p[0] = 1;
    for i in 0..MAX_SIZ - 1 {
        len[i + 1] = 2 * len[i] + 3;
        p[i + 1] = 2 * p[i] + 1;
    }

    let ans = rec(N, X, &len, &p);

    println!("{}", ans);
}

// 残りx段の中にパティが何枚含まれるか
fn rec(n: usize, x: usize, len: &Vec<usize>, p: &Vec<usize>) -> usize {
    debug!(n, x);
    let res = if n == 0 {
        if x <= 0 {
            0
        } else {
            1
        }
    } else if x >= 2 * len[n - 1] + 2 {
        2 * p[n - 1] + 1
    } else if x >= len[n - 1] + 2 {
        rec(n - 1, x - len[n - 1] - 2, len, p) + p[n - 1] + 1
    } else if x >= len[n - 1] + 1 {
        rec(n - 1, x - len[n - 1] - 1, len, p) + p[n - 1]
    } else if x > 0 {
        rec(n - 1, x - 1, len, p)
    } else {
        0
    };
    debug!(res);
    res
}
