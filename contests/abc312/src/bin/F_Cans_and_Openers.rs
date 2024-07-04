//           F - Cans and Openers          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc312/tasks/abc312_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::cmp::Reverse;

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
        M: usize,
        cans: [(usize, usize); N]
    }

    let mut a = vec![];  // 缶切りのいらない缶
    let mut b = vec![];  // 缶切りが必要な缶
    let mut c = vec![];  // 缶切り
    for &(t, x) in &cans {
        match t {
            0 => a.push(x),
            1 => b.push(x),
            _ => c.push(x),
        }
    }

    // ソートしておく
    a.sort_by_key(|&v| Reverse(v));
    b.sort();
    c.sort();

    // 缶切りのいらない缶を大きい順にn個とったときの価値
    let mut x = vec![0; N + 1];
    for i in 0..N {
        if i < a.len() {
            x[i + 1] = x[i] + a[i];
        } else {
            x[i + 1] = x[i];
        }
    }

    // 缶切りと缶切りのいる缶を合わせてn個とったときの価値
    let mut y = vec![0; N + 1];
    let mut can_open = 0;
    for i in 0..N {
        // 缶がない時
        if b.is_empty() {
            y[i + 1] = y[i];
        }
        // 缶切りがない時
        else if can_open == 0 {
            if let Some(c) = c.pop() {
                can_open += c;
            }
            y[i + 1] = y[i];
        }
        // 開封できる時
        else {
            y[i + 1] = y[i] + b.pop().unwrap();
            can_open -= 1;
        }
    }

    debug!(&x);
    debug!(&y);

    // 合計がM個になるようにとった時の価値の最大値
    let mut ans = 0;
    for i in 0..=M {
        ans = ans.max(
            x[i] + y[M - i]
        );
    }

    println!("{ans}");
}
