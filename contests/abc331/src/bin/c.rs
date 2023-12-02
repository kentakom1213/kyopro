// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

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

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // ソート
    let sorted = A.iter().cloned().sorted().collect_vec();

    // 累積和
    let S = (0..N).fold(vec![0; N + 1], |mut s, i| {
        s[i + 1] = s[i] + sorted[i];
        s
    });

    // にぶたん
    for i in 0..N {
        let ub = sorted.upper_bound(&A[i]);
        let res = S[N] - S[ub];
        print!("{} ", res);
    }
    println!();
}
