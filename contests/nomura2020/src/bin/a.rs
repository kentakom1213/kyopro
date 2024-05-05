// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        H1: usize,
        M1: usize,
        H2: usize,
        M2: usize,
        K: usize,
    }

    // 分に直す
    let m1 = H1 * 60 + M1;
    let m2 = H2 * 60 + M2;

    let ans = (m2 - m1).saturating_sub(K);

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
