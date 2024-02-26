#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut R: [usize; N],
        queries: [usize; Q]
    }

    R.sort();
    // 累積和
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + R[i];
    }

    debug!(S);

    for &q in &queries {
        let res = S.upper_bound(&q) - 1;
        println!("{res}");
    }
}
