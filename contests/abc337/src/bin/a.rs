#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
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
        XY: [(usize, usize); N]
    }

    let mut sumX = 0;
    let mut sumY = 0;
    
    for &(x, y) in &XY {
        sumX += x;
        sumY += y;
    }

    if sumX > sumY {
        println!("Takahashi");
    } else if sumX < sumY {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
