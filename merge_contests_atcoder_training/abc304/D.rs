// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        W: usize,
        H: usize,
        N: usize,
        straw: [(usize, usize); N],
        a: usize,
        mut A: [usize; a],
        b: usize,
        mut B: [usize; b],
    }

    // いちごを座標圧縮
    // let (mut straw_x, mut straw_y): (Vec<_>, Vec<_>) = straw.iter().cloned().unzip();
    // straw_x.sort();
    // straw_x.dedup();
    // straw_y.sort();
    // straw_y.dedup();

    // debug!(&straw_x, &straw_y);

    // Aを座標圧縮
    A.push(W);
    B.push(H);

    debug!(&A, &B);

    // 保存する
    let mut cnt = BTreeMap::new();

    // いちごを区画に保存
    for &(x, y) in &straw {
        let x_idx = A.lower_bound(&x);
        let y_idx = B.lower_bound(&y);
        // 追加
        *cnt.entry((x_idx, y_idx)).or_insert(0) += 1;
    }

    debug!(&cnt);

    // 区画が少ない→全探索
    let max = cnt.iter().map(|(k, &v)| v).max().unwrap();

    // 0にならない可能性がある場合
    let min = if (a + 1) * (b + 1) <= N {
        let mut has_blank = false;
        for x in 0..=a {
            for y in 0..=b {
                has_blank |= !cnt.contains_key(&(x, y));
            }
        }
        if has_blank {
            0
        } else {
            cnt.iter().map(|(k, &v)| v).min().unwrap()
        }
    } else {
        0
    };

    println!("{} {}", min, max);
}
