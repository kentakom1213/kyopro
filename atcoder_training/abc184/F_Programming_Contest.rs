//         F - Programming Contest         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc184/tasks/abc184_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

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

// main
fn main() {
    input! {
        N: usize,
        T: usize,
        A: [usize; N],
    }

    // 半分に分割
    let L = &A[.. N / 2];
    let R = &A[N / 2 ..];
    let (ll, rl) = (L.len(), R.len());

    // 右半分をsetに格納
    let mut right = BTreeSet::new();
    for i in 0..1 << rl {
        let mut tmp = 0;
        for j in 0..rl {
            if i >> j & 1 == 1 {
                tmp += R[j];
            }
        }
        right.insert(tmp);
    }

    // 左半分を全探索
    let mut ans = 0;
    for i in 0..1 << ll {
        let mut tmp = 0;
        for j in 0..ll {
            if i >> j & 1 == 1 {
                tmp += L[j];
            }
        }
        // tmp + x <= T となる最大のxを求める
        if tmp <= T {
            let x = right.range(..=T - tmp).next_back().unwrap();
            ans = ans.max(
                tmp + x
            );
        }
    }

    println!("{}", ans);
}
