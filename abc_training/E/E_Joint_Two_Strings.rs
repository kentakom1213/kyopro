//          E - Joint Two Strings          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc324/tasks/abc324_e
// ----------------------------------------

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

    let SIZE = T.len();
    let mut pre = vec![0; SIZE + 1];
    let mut suf = vec![0; SIZE + 1];
    for i in 0..N {
        pre[prefix[i]] += 1;
        suf[suffix[i]] += 1;
    }

    debug!(&pre);
    debug!(&suf);

    // pre[i] + suf[j] >= T.len() を満たす(i,j)の個数が答えとなる
    let mut acc = 0_usize;
    let mut ans = 0;
    for i in 0..=SIZE {
        acc += suf[SIZE - i];
        ans += pre[i] * acc;
    }
    println!("{}", ans);
}

fn cnt_prefix(T: &Vec<char>, S: &Vec<char>) -> usize {
    let mut i = 0;
    let mut j = 0;
    while i < T.len() && j < S.len() {
        if T[i] == S[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    i
}

