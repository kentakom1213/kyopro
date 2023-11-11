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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars,
        queries: [(Usize1, Usize1); Q]
    }
    // 隣り合うかどうか
    let mut is_same = vec![0; N - 1];
    for i in 0..N - 1 {
        if S[i] == S[i + 1] {
            is_same[i] += 1;
        }
    }

    debug!(is_same);

    // 隣り合う箇所の数の累積和
    let mut sum = vec![0; N];
    for i in 0..N - 1 {
        sum[i + 1] = sum[i] + is_same[i];
    }

    debug!(sum);

    for &(l, r) in &queries {
        println!("{}", sum[r] - sum[l]);
    }
}
