// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
        ranges: [(usize, usize); Q],
    }

    // 累積和
    let mut acc = vec![0; N + 1];
    for (i, c) in S.chars().enumerate() {
        if i > 0 {
            if &S[i - 1..i + 1] == "AC" {
                acc[i + 1] = acc[i] + 1;
            }
            else {
                acc[i + 1] = acc[i];
            }
        }
    }

    debug!(&acc);

    for &(l, r) in &ranges {
        println!("{}", acc[r] - acc[l]);
    }
}
