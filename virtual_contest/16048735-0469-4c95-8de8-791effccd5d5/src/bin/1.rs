// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;

// imports
use itertools::Itertools;
use proconio::{
    input,
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

fn main() {
    input! {
        N: usize,
        S: [String; N],
        M: usize,
        T: [String; M],
    }

    let mut cnt = HashMap::new();

    for s in S {
        *cnt.entry(s).or_insert(0) += 1;
    }

    for t in T {
        *cnt.entry(t).or_insert(0) -= 1;
    }

    let mut ans = 0;

    for (_, &v) in &cnt {
        if v > ans {
            ans = v;
        }
    }

    println!("{ans}");
}
