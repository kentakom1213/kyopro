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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    // 半開区間で考える
    input! {
        A: usize,
        mut B: usize,
    }
    B += 1;

    let mut ans = 0;

    // 桁ごとに考える
    for d in 0..60 {
        let cnt = 1 << d; // 繰り返しあたりに含まれる1の個数
        let wid = 1 << d + 1; // 繰り返しの幅

        // Aまでに含まれる1の個数
        let bef = A / wid * cnt + (A % wid).saturating_sub(cnt);
        // Bまでに含まれる1の個数
        let aft = B / wid * cnt + (B % wid).saturating_sub(cnt);
        debug!(d, bef, aft);
        ans |= (aft - bef & 1) << d;
    }

    println!("{}", ans);
}
