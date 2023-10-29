//            D - Cutting Woods            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc217/tasks/abc217_d
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
#[fastout]
fn main() {
    input! {
        L: usize,
        Q: usize,
    }

    let mut bst = BTreeSet::new();
    bst.insert(0);
    bst.insert(L);

    for _ in 0..Q {
        input! {
            c: usize,
            x: usize,
        }

        if c == 1 {
            bst.insert(x);
        } else {
            let l = bst.range(..x).next_back().unwrap();
            let r = bst.range(x..).next().unwrap();
            println!("{}", r - l);
        }
    }
}
