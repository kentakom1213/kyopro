//               D - Cake 123
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc123/tasks/abc123_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
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
const NEG1: usize = 1_usize.wrapping_neg();

fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [usize; X],
        B: [usize; Y],
        C: [usize; Z],
    }

    // A, Bの直積
    let AxB: Vec<usize> = iproduct!(A.iter(), B.iter())
        .map(|(&a, &b)| a + b)
        .sorted() // ソート
        .skip((X * Y).saturating_sub(K)) // 上位K個を取り出す
        .collect();

    debug!(&AxB);

    // AxBxCを考える
    let AxBxC: Vec<usize> = iproduct!(AxB.iter(), C.iter())
        .map(|(&ab, &c)| ab + c)
        .sorted()
        .rev()
        .take(K)
        .collect();

    println!("{}", AxBxC.iter().join("\n"));
}
