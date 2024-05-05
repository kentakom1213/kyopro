// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use libm::pow;
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

/// 余りをとる累乗
pub fn powusize(mut a: usize, mut b: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a);
        }
        a = (a * a);
        b >>= 1;
    }
    res
}

fn main() {
    input! {
        B: usize,
    }

    for a in 1..20 {
        let x = powusize(a, a);
        if x == B {
            println!("{}", a);
            return;
        }
    }
    println!("-1");
}
