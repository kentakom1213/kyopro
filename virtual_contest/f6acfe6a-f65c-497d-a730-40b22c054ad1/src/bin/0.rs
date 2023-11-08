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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        W: usize,
        H: usize,
        N: usize,
        dots: [(usize, usize, usize); N],
    }

    let l = *dots
        .iter()
        .filter(|(_, _, a)| *a == 1)
        .map(|(x, _, _)| x)
        .max()
        .unwrap_or(&0);
    let r = *dots
        .iter()
        .filter(|(_, _, a)| *a == 2)
        .map(|(x, _, _)| x)
        .min()
        .unwrap_or(&W);
    let b = *dots
        .iter()
        .filter(|(_, _, a)| *a == 3)
        .map(|(_, y, _)| y)
        .max()
        .unwrap_or(&0);
    let t = *dots
        .iter()
        .filter(|(_, _, a)| *a == 4)
        .map(|(_, y, _)| y)
        .min()
        .unwrap_or(&H);

    debug!(l, r, b, t);

    let ans = r.saturating_sub(l) * t.saturating_sub(b);

    println!("{}", ans);
}
