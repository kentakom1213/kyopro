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
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    
    let ans = a.abs_diff(c) + b.abs_diff(d) + 1;
    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
