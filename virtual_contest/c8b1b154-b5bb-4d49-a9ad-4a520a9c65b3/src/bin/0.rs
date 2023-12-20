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
        N: usize,
        X: usize,
        S: String
    }

    let mut bits = format!("{:b}", X);

    for c in S.chars() {
        match c {
            'U' => {
                bits.pop();
            }
            'L' => {
                bits.push('0');
            }
            'R' => {
                bits.push('1');
            }
            _ => (),
        }
    }

    let ans = usize::from_str_radix(&bits, 2).unwrap();

    println!("{}", ans);
}
