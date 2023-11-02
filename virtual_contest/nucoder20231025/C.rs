// https://atcoder.jp/contests/abc030/tasks/abc030_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::str::FromStr;

use im_rc::HashMap;
// imports
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive, Num};
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
        a: Usize1,
        k: String,
        B: [Usize1; N],
    }

    let K = BigUint::from_str_radix(&k, 10).unwrap();
    debug!(&K);

    // ループ検出
    let mut cnt = 0_usize;
    let mut cur = a;
    let mut visit = HashMap::new();
    while visit.get(&cur).is_none() {
        let nxt = B[cur];
        visit.insert(cur, (nxt, cnt));
        cur = nxt;
        cnt += 1;
    }
    let (_, loop_begin) = visit[&cur];
    let loop_len = cnt - loop_begin;

    debug!(loop_begin, loop_len);

    let loop_begin_big = BigUint::from_usize(loop_begin).unwrap();
    let loop_len_big = BigUint::from_usize(loop_len).unwrap();

    // Kが十分に短いとき
    if K > loop_begin_big {
        let rem = (K - loop_begin_big) % loop_len_big;
        // 愚直に回す
        let mut cur = cur;
        for _ in 0..rem.to_usize().unwrap() {
            cur = B[cur];
        }
        println!("{}", cur + 1);
    } else {
        // 愚直に回す
        let mut cur = a;
        for _ in 0..K.to_usize().unwrap() {
            cur = B[cur];
        }
        println!("{}", cur + 1);
    }
}
