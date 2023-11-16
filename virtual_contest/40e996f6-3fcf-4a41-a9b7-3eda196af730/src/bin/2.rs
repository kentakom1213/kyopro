// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeSet, BTreeMap};

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

// #[fastout]
fn main() {
    input! {
        H: isize,
        W: isize,
        rs: isize,
        cs: isize,
        N: usize,
        blocks: [(isize, isize); N],
        Q: usize,
    }

    // 行ごと
    let mut rows: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();
    // 列ごと
    let mut cols: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();

    // ブロックを配置
    for &(r, c) in &blocks {
        rows.entry(r).or_insert_with(|| BTreeSet::new()).insert(c);
        cols.entry(c).or_insert_with(|| BTreeSet::new()).insert(r);
    }

    debug!(rows);
    debug!(cols);

    // クエリの処理
    let mut r = rs;
    let mut c = cs;

    for _ in 0..Q {
        input! {
            d: char,
            l: isize,
        }
        match d {
            'L' => {
                if let Some(row) = rows.get(&r) {
                    if let Some(&left) = row.range(..c).next_back() {
                        c = left + 1;
                    } else {
                        c = 1.max(c - l);
                    }
                } else {
                    c = 1.max(c - l);
                }
            }
            'R' => {
                if let Some(row) = rows.get(&r) {
                    if let Some(&right) = row.range(c..).next() {
                        c = right - 1;
                    } else {
                        c = W.min(c + l);
                    }
                } else {
                    c = W.min(c + l);
                }
            }
            'U' => {
                if let Some(col) = cols.get(&c) {
                    if let Some(&top) = col.range(..r).next_back() {
                        r = top + 1;
                    } else {
                        r = 1.max(r - l);
                    }
                } else {
                    r = 1.max(r - l);
                }
            }
            'D' => {
                if let Some(col) = cols.get(&c) {
                    if let Some(&bottom) = col.range(r..).next() {
                        r = bottom - 1;
                    } else {
                        r = H.min(r + l);
                    }
                } else {
                    r = H.min(r + l);
                }
            }
            _ => ()
        }
        println!("{} {}", r, c);
    }
}
