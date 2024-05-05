// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BinaryHeap};

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        A: [usize; N],
        B: [usize; M],
        CD: [(Usize1, Usize1); L]
    }

    let NG: FxHashSet<_> = CD.iter().cloned().collect();

    // Bを降順にソート
    let ordB = (0..M).sorted_by_key(|&i| Reverse(B[i])).collect_vec();

    // Bのどこまでを見たか
    let mut cur = vec![0; N];

    // 候補となる組
    let mut q: BinaryHeap<_> = A
        .iter()
        .enumerate()
        .map(|(c, &a)| (a + B[ordB[0]], c))
        .collect();

    while let Some((sum, c)) = q.pop() {
        let d = cur[c];
        if NG.contains(&(c, ordB[d])) {
            cur[c] += 1;
            q.push((A[c] + B[ordB[cur[c]]], c));
        } else {
            println!("{}", sum);
            return;
        }
    }
}

const INF: usize = 1001001001001001001;
