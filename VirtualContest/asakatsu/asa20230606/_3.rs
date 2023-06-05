// https://atcoder.jp/contests/abc174/tasks/abc174_d

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
    input! {
        N: usize,
        mut C: Chars,
    }

    // 左側のポインタ、右側のポインタ
    let (mut i, mut j) = (0, N - 1);
    let mut ans = 0;

    while i < j {
        match (C[i], C[j]) {
            ('R', 'R') => {
                i += 1;
            },
            ('R', 'W') => {
                i += 1;
                j -= 1;
            },
            ('W', 'R') => {
                // 交換
                C.swap(i, j);
                ans += 1;
            },
            ('W', 'W') => {
                j -= 1;
            },
            _ => (),
        }
        debug!(i, j, &C);
    }

    println!("{}", ans);
}
