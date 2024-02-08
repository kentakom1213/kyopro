//                D - Base n
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc192/tasks/abc192_d
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

// main
fn main() {
    input! {
        X: String,
        M: usize,
    }

    let X = X
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    if X.len() == 1 {
        let d = X[0];
        if d <= M {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    let d = *X.iter().max().unwrap();

    // 2分探索
    let mut ok = 1;
    let mut ng = M + 1;
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        debug!(ok, m, ng, as_base_n(m, &X));
        if as_base_n(m, &X) <= M {
            ok = m;
        } else {
            ng = m;
        }
    }

    let ans = ok.saturating_sub(d);
    println!("{}", ans);
}

/// 基数変換
fn as_base_n(n: usize, X: &Vec<usize>) -> usize {
    let mut res = 0_usize;
    for &d in X {
        res = res.saturating_mul(n);
        res = res.saturating_add(d);
    }
    res
}
