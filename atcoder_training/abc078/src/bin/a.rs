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
use std::cmp::Ordering::*;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        X: char,
        Y: char,
    }

    let ans = match X.cmp(&Y) {
        Less => '<',
        Equal => '=',
        Greater => '>',
    };
    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
