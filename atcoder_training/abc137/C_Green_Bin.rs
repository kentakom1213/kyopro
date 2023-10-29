//              C - Green Bin              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc137/tasks/abc137_c
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

// main
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut ans: usize = 0;
    let mut map = BTreeMap::new();

    // ソートして処理
    for s in &S {
        let ss = s.chars().sorted().join("");
        let val = map.entry(ss).or_insert(0);
        ans += *val;
        *val += 1;
    }

    println!("{}", ans);
}

