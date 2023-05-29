//              C - All Green
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc104/tasks/abc104_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
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

// main
fn main() {
    input! {
        D: usize,
        G: usize,
        probs: [(usize, usize); D],
    }

    // 問題を点数が大きい順にソートしておく
    let probs = probs
        .iter()
        .enumerate()
        .map(|(i, &(p, c))| (100 * (i + 1), p, c))
        .sorted()
        .rev()
        .collect_vec();
    debug!(&probs);

    // bit全探索
    let mut ans = INF;
    for i in 0..1 << D {
        let mut tmp = 0; // 現在といた問題数
        let mut g = G; // 残っているスコア
        let mut rem = vec![]; // 残っている問題
        for j in 0..D {
            let (t, p, c) = probs[j];
            if i >> j & 1 == 1 {
                g = g.saturating_sub(t * p + c);
                tmp += p;
            } else {
                rem.push((t, p - 1)); // 点数, 残っている問題数
            }
        }
        debug!(tmp, g, &rem);
        // 残っているスコアを減らしていく
        for &(t, p) in &rem {
            if g == 0 {
                break;
            }
            // 解くべき問題数
            let need = (g + t - 1) / t;
            tmp += need.min(p);
            g = g.saturating_sub(t * need.min(p));
        }
        if g == 0 {
            ans = ans.min(tmp);
        }
    }

    println!("{}", ans);
}
