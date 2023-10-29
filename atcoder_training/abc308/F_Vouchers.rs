//               F - Vouchers              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc308/tasks/abc308_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BinaryHeap;

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

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        P: [usize; N],
        L: [usize; M],
        D: [usize; M],
    }

    // LDの組をソートしておく
    let LD = L.iter().cloned().zip(D.iter().cloned()).sorted().collect_vec();

    // 貪欲に処理
    let mut ans = 0;
    let mut ptr = 0;
    let mut pq = BinaryHeap::new();

    for &p in P.iter().sorted() {
        // Lを超えない範囲で追加
        while ptr < M && LD[ptr].0 <= p {
            pq.push(LD[ptr].1);
            ptr += 1;
        }
        // 答えに加算
        ans += p;
        if let Some(d) = pq.pop() {
            ans -= d;
        }
    }

    println!("{}", ans);
}
