// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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
        A: [usize; N],
    }

    // 左ピラミッドの最大値
    let mut L = vec![0; N];
    L[0] = 1;

    for i in 1..N {
        if A[i] > L[i - 1] {
            L[i] = L[i - 1] + 1;
        } else {
            L[i] = A[i];
        }
    }

    debug!(L);

    // 右ピラミッドの最大値
    let mut R = vec![0; N];
    R[N - 1] = 1;

    for i in (0..N - 1).rev() {
        if A[i] > R[i + 1] {
            R[i] = R[i + 1] + 1;
        } else {
            R[i] = A[i];
        }
    }

    debug!(R);

    let ans = L.iter().zip(R.iter()).map(|(a, b)| a.min(b)).max().unwrap();

    println!("{ans}");
}
