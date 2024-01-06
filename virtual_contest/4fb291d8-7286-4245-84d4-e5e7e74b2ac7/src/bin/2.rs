// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use alga::general::Lattice;
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
const INF: usize = 100100100100100100;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let time = |x| {
        let g = (1 + x) as f64;
        a as f64 / g.sqrt() + b as f64 * x as f64
    };

    // 3分探索
    let (mut l, mut r) = (0, INF);

    while r - l > 2 {
        let m1 = l + (r - l) / 3;
        let m2 = r.saturating_sub((r - l) / 3);
        if time(m1) < time(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }

    let ans = (l..=r)
        .map(|x| time(x))
        .fold(f64::MAX, |m, x| *m.partial_min(&x).unwrap());

    println!("{:.20}", ans);
}
