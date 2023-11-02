// https://atcoder.jp/contests/abc107/tasks/arc101_a

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: [isize; N],
    }

    // 左からK個とる
    let mut deq = VecDeque::new();
    for &v in &X[..K] {
        deq.push_back(v);
    }

    let mut ans = INF as isize;

    let l = deq.front().unwrap();
    let r = deq.back().unwrap();
    // 現在のスコア
    chmin! {
        ans,
        l.abs() + (r - l).abs(),
        r.abs() + (r - l).abs()
    };

    debug!(&deq);

    for i in 0..N - K {
        deq.pop_front();
        deq.push_back(X[K + i]);
        debug!(&deq);

        let l = deq.front().unwrap();
        let r = deq.back().unwrap();
        // 現在のスコア
        chmin! {
            ans,
            l.abs() + (r - l).abs(),
            r.abs() + (r - l).abs()
        };
    }

    println!("{}", ans);
}
