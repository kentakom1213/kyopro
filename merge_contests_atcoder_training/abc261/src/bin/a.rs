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
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    }

    let ans = r1.min(r2).saturating_sub(l1.max(l2));

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
