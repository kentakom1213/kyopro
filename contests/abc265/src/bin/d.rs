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
        N: usize,
        P: usize,
        Q: usize,
        R: usize,
        A: [usize; N]
    }

    // 累積和
    let S = (0..N).fold(vec![0; N + 1], |mut s, i| {
        s[i + 1] = s[i] + A[i];
        s
    });

    
}

const INF: usize = 1001001001001001001;
