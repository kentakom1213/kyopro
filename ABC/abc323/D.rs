// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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
        slimes: [(usize, usize); N],
    }

    let mut map = BTreeMap::new();

    for &(s, c) in &slimes {
        map.insert(s, c);
    }

    let mut ans = 0_usize;

    while let Some((s, c)) = map.pop_first() {
        // あまりを足す
        ans += c % 2;
        if c / 2 == 0 {
            continue;
        }
        // 2をかけて増やす
        *map.entry(s * 2).or_insert(0) += c / 2;
    }

    println!("{}", ans);
}
