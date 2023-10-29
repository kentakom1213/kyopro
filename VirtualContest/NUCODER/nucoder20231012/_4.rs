// https://atcoder.jp/contests/abc085/tasks/abc085_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::BinaryHeap;

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
        mut H: usize,
        katanas: [(usize, usize); N]
    }

    let mut pq = BinaryHeap::new();

    // 刀を追加
    for &(a, b) in &katanas {
        // 何度でも使える
        pq.push((a, true));
        // 1回しか使えない
        pq.push((b, false));
    }

    let mut cnt = 0;

    while let Some(&(v, can_loop)) = pq.peek() {
        if can_loop {
            // 無限に攻撃できる
            cnt += (H + v - 1) / v;
            H = 0;
        } else {
            // 1回だけ攻撃
            cnt += 1;
            H = H.saturating_sub(v);
            pq.pop();
        }
        debug!(cnt);
        if H == 0 {
            break;
        }
    }

    println!("{cnt}");
}
