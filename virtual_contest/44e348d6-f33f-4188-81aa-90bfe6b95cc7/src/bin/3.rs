// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        A: [usize; N]
    }

    // しゃくとり法で単調増加で極大な部分列
    let mut ans = 0_usize;
    let mut deq = VecDeque::new();

    for &a in &A {
        if !deq.is_empty() && deq.back().unwrap() >= &a {
            deq.clear();
        }
        deq.push_back(a);
        ans += deq.len();
        debug!(deq);
    }

    println!("{}", ans);
}
