// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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
        mut T: Chars,
        mut S: [Chars; N],
    }

    // SがTの何文字目までを含むか
    let mut prefix = vec![0; N];
    for (i, s) in S.iter_mut().enumerate() {
        prefix[i] = cnt_prefix(&T, s);
        // 反転させる
        s.reverse();
    }

    T.reverse();

    // SがTの後ろから何文字目までをふくむか
    let mut suffix = vec![0; N];
    for (i, s) in S.iter().enumerate() {
        suffix[i] = cnt_prefix(&T, s);
    }

    debug!(&prefix);
    debug!(&suffix);
}

fn cnt_prefix(T: &Vec<char>, S: &Vec<char>) -> usize {
    let mut cnt = 0;
    let mut i = 0;
    for t in T.iter() {
        while i < S.len() && t != &S[i] {
            i += 1;
        }
        cnt += 1;
        i += 1;
        if i >= S.len() {
            break;
        }
    }
    cnt
}
