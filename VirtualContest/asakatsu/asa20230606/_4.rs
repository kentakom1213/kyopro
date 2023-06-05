// https://atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_b

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::collections::HashSet;

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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    for i in 0..=N {
        for j in 0..=M {
            let tmp = M * i + N * j - 2 * i * j;
            debug!(M * i, N * j, i * j);
            debug!(tmp);
            if tmp == K {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
