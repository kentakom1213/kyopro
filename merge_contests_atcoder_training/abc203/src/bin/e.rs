#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    result,
};

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

fn main() {
    input! {
        N: isize,
        M: usize,
        XY: [(isize, isize); M]
    }

    // 候補となるc座標
    let mut S = FxHashSet::from_iter([N]);

    // r座標が小さい順に見ていく
    for (&g, rcs) in &XY.iter().sorted().group_by(|&(r, c)| r) {
        // Sに追加する座標の集合
        let mut A = FxHashSet::default();

        // Sから削除する座標の集合
        let mut B = FxHashSet::default();

        for &(r, c) in rcs {
            // 左上，右上から到達できる場合
            if (S.contains(&(c - 1)) || S.contains(&(c + 1))) && !S.contains(&c) {
                A.insert(c);
            }
            // 上からしか到達できない場合，削除
            if (!S.contains(&(c - 1)) && !S.contains(&(c + 1))) && S.contains(&c) {
                B.insert(c);
            }
        }

        // Sを更新
        for &a in &A {
            S.insert(a);
        }

        for &b in &B {
            S.remove(&b);
        }
    }

    println!("{}", S.len());
}

const INF: usize = 1001001001001001001;
