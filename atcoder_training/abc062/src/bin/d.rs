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
    collections::{BTreeSet, BinaryHeap},
};

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
        A: [isize; 3 * N],
    }

    // 先頭の候補
    let mut Sx = vec![0; N + 1];
    let mut X: BinaryHeap<Reverse<isize>> = A[..N]
        .iter()
        .inspect(|&&v| Sx[0] += v)
        .map(|&v| Reverse(v))
        .collect();

    let mut Sy = vec![0; N + 1];
    let mut Y: BinaryHeap<isize> = A[2 * N..]
        .iter()
        .inspect(|&&v| Sy[N] += v)
        .cloned()
        .collect();

    // 前半、後半の区切りを全探索
    for k in N..2 * N {
        debug!(X, k);
        // XにAkを追加（既存の要素より大きい場合）
        if X.peek().unwrap().0 < A[k] {
            let old = X.pop().unwrap().0;
            X.push(Reverse(A[k]));
            // 更新
            Sx[k - N + 1] = Sx[k - N] - old + A[k];
        } else {
            Sx[k - N + 1] = Sx[k - N];
        }
    }

    for k in (N..2 * N).rev() {
        debug!(Y, k);
        // YにAkを追加（既存の要素より小さい場合）
        if *Y.peek().unwrap() > A[k] {
            let old = Y.pop().unwrap();
            Y.push(A[k]);
            // 更新
            Sy[k - N] = Sy[k + 1 - N] - old + A[k];
        } else {
            Sy[k - N] = Sy[k + 1 - N];
        }
    }

    debug!(Sx, Sy);

    let ans = (0..=N).map(|k| Sx[k] - Sy[k]).max().unwrap();

    println!("{ans}");
}

const INF: isize = 1001001001001001001;
