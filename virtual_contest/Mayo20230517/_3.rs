// https://atcoder.jp/contests/abc198/tasks/abc198_c

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_traits::Pow;
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

// main
fn main() {
    input! {
        R: f64,
        X: f64,
        Y: f64,
    }
    
    // 自明な下界を求める
    let D = (X.powf(2.0) + Y.powf(2.0)).sqrt();
    let lb = (D / R).ceil();

    if D < R {
        println!("2");
    } else {
        println!("{}", lb);
    }
}
