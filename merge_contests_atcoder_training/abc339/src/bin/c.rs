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
        A: [isize; N]
    }

    // 累積和
    let mut S = vec![0; N + 1];
    for (i, &a) in A.iter().enumerate() {
        S[i + 1] = S[i] + a;
        if S[i + 1] < 0 {
            S[i + 1] = 0;
        }
    }

    debug!(S);

    println!("{}", S.last().unwrap());
}
