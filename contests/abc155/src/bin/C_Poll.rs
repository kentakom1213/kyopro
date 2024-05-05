//                 C - Poll                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc155/tasks/abc155_c
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
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut map = BTreeMap::new();
    let mut max_count = 0;

    for s in &S {
        *map.entry(s).or_insert(0) += 1;
        chmax!(
            max_count,
            map[s],
        );  
    }

    for (k, &v) in &map {
        if v == max_count {
            println!("{}", k);
        }
    }
}
