// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars,
        queries: [(usize, usize); Q]
    }

    let mut begin = 0;

    for &(t, x) in &queries {
        if t == 1 {
            begin = (N + begin - x) % N;
        }
        else {
            let idx = (begin + x - 1) % N;
            println!("{}", S[idx]);
        }
        debug!(begin);
    }
}
