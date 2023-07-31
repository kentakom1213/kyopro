//            C - Invisible Hand
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc312/tasks/abc312_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
        mut B: [usize; M],
    }

    A.sort();
    B.sort();

    // 累積和
    let mut SA = vec![0; N + 1];
    for (i, &a) in A.iter().enumerate() {
        SA[i + 1] = SA[i] + a;
    }

    let mut SB = vec![0; M + 1];
    for (i, &b) in B.iter().enumerate() {
        SB[i + 1] = SB[i] + b;
    }

    // 二分探索
    let mut ng = 0;
    let mut ok = INF;

    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        // 座標をもどす
        let ma = A.lower_bound(&mid);
        let mb = B.lower_bound(&mid);

    }

    println!("{}", ok);
}
