// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use ac_library::maxflow::*;
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
        C: usize,
        A: [usize; N],
        B: [usize; N],
    }

    // 列の切り方で全探索
    for d in 0..1_usize << (N - 1) {
        // let comb = vec![];
        let mut tmp = 0;
        for i in 0..N - 1 {
            if d >> i & 1 == 1 {

            }
        }
    }
}
