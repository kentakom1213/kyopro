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
        T: Chars,
        S: [Chars; N],
    }

    let mut ans = vec![];

    for i in 0..N {
        let s = &S[i];
        if get_dist(&s, &T) <= 1 || is_insert(&s, &T) || is_insert(&T, &s) {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}

fn get_dist(S: &Vec<char>, T: &Vec<char>) -> usize {
    if S.len() != T.len() {
        return INF;
    }
    let mut cnt = 0;
    for (s, t) in S.iter().zip(T.iter()) {
        if s != t {
            cnt += 1;
        }
    }
    cnt
}

/// SはTに一文字挿入して得られる文字列か
fn is_insert(S: &Vec<char>, T: &Vec<char>) -> bool {
    if S.len() != T.len() + 1 {
        return false;
    }
    let mut cnt = 0;

    let mut idx = 0;
    for t in T.iter() {
        if t != &S[idx] {
            if cnt == 0 && idx + 1 < S.len() && t == &S[idx + 1] {
                idx += 1;
                cnt += 1;
            } else {
                return false;
            }
        }
        idx += 1;
    }
    cnt <= 1
}
