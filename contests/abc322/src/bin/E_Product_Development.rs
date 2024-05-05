//         E - Product Development
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc322/tasks/abc322_e
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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
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
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        P: usize,
    }

    let mut A = vec![];
    let mut C = vec![];
    for _ in 0..N {
        input! {
            c: isize,
            a: [usize; K],
        }
        C.push(c);
        A.push(a);
    }

    // dp[params] := paramsを達成するための最小のコスト
    let mut dp = BTreeMap::new();
    dp.insert(vec![0; K], 0_isize);

    debug!(&dp);

    // 組合せは高々 P^K <= 3125通り
    // 全探索すれば良い
    for (params, &cost) in A.iter().zip(C.iter()) {
        for (cur_params, &cur_cost) in &dp.clone() {
            let mut new_params = vec![0; K];
            for i in 0..K {
                new_params[i] = P.min(params[i] + cur_params[i]);
            }
            let val = dp.entry(new_params).or_insert(INF);
            chmin!(*val, cur_cost + cost);
        }
    }

    debug!(&dp);

    let ans = dp.entry(vec![P; K]).or_insert(-1);

    println!("{}", ans);
}
