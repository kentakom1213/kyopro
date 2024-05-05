//           D - Printing Machine
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc325/tasks/abc325_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

// imports
use itertools::Itertools;
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
        TD: [(usize, usize); N],
    }

    let TD: Vec<(usize, usize)> = TD.iter().map(|&(t, d)| (t, t + d)).sorted().collect();

    // 商品を管理
    let mut pq = BinaryHeap::new();

    // 時刻
    let mut t = 0_usize;

    // 何番目までみたか
    let mut nxt = 0;

    // 答えを管理
    let mut ans = 0;

    loop {
        t += 1;

        // 商品がない場合
        if pq.is_empty() {
            if nxt == N {
                break;
            }
            t = TD[nxt].0;
        }
        // 同じ時刻に始まるものをヒープに突っ込む
        while nxt < N && TD[nxt].0 == t {
            pq.push(Reverse(TD[nxt].1));
            nxt += 1;
        }
        debug!(t, &pq);
        // 使えないものを削除
        while !pq.is_empty() && pq.peek().unwrap().0 < t {
            pq.pop();
        }
        debug!(t, &pq);
        // 要素が残っている場合には，先頭のものを取り出す
        if pq.pop().is_some() {
            ans += 1;
        }
    }

    println!("{}", ans);
}
