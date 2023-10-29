//               C - Flavors
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc315/tasks/abc315_c
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
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
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

// main
fn main() {
    input! {
        N: usize,
        ice: [(Usize1, usize); N],
    }

    let mut flavors = vec![vec![]; N];

    for &(f, s) in &ice {
        flavors[f].push(s);
    }

    for i in 0..N {
        flavors[i].sort();
        flavors[i].reverse();
    }

    // 同じ味を選ぶ時
    let mut same = 0;

    for i in 0..N {
        if flavors[i].len() >= 2 {
            chmax!(same, flavors[i][0] + flavors[i][1] / 2);
        }
    }

    // 違う味を選ぶ時
    let mut maxes = flavors
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| v[0])
        .collect::<Vec<usize>>();

    maxes.sort();
    maxes.reverse();

    let diff = if maxes.len() >= 2 { maxes[0] + maxes[1] } else { 0 };

    let ans = same.max(diff);

    println!("{}", ans);
}
