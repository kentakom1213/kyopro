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
use num_traits::{FromPrimitive, Num, ToPrimitive};
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

fn main() {
    input! {
        N: usize,
        a: Usize1,
        k: usize,
        B: [Usize1; N],
    }

    // ループ検出
    let mut cnt = 0_usize;
    let mut cur = a;
    let mut visit = HashMap::new();
    while visit.get(&cur).is_none() {
        visit.insert(cur, cnt);
        cur = B[cur];
        cnt += 1;
    }
    let loop_begin = visit[&cur];
    let loop_len = cnt - loop_begin;

    debug!(loop_begin, loop_len);

    // Kが十分に短いとき
    if k > loop_begin {
        let rem = (k - loop_begin) % loop_len;
        // 愚直に回す
        let mut cur = loop_begin;
        for _ in 0..rem.to_usize().unwrap() {
            cur = B[cur];
        }
        println!("{}", cur + 1);
    } else {
        // 愚直に回す
        let mut cur = a;
        for _ in 0..k {
            cur = B[cur];
        }
        println!("{}", cur + 1);
    }
}
