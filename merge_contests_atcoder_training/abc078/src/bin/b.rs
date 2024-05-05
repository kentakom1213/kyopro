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
        X: usize,
        Y: usize,
        Z: usize
    }

    // let div_ceil = |a, b| {
    //     (a + b - 1) / b
    // };

    let cnt = X / (Y + Z);

    debug!(cnt * (Y + Z));

    if X >= cnt * (Y + Z) + Z {
        println!("{}", cnt);
    } else {
        println!("{}", cnt - 1);
    }
}

const INF: usize = 1001001001001001001;
