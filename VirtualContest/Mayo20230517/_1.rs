// https://atcoder.jp/contests/abc129/tasks/abc129_b

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// main
fn main() {
    input! {
        N: usize,
        W: [usize; N],
    }

    // 累積和
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + W[i];
    }

    // 答え
    let mut ans = INF;
    for i in 0..N {
        let l = S[i];
        let r = S[N] - S[i];
        let diff = if l < r { r - l } else { l - r };
        if ans > diff {
            ans = diff;
        }
    }

    println!("{}", ans);
}
