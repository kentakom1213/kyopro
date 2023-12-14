// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
        mut A: [Usize1; N]
    }

    // ダブリング
    for _ in 0..30 {
        A = A.iter().map(|&a| A[a]).collect();
    }

    let ans = HashSet::<usize>::from_iter(A).len();

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
