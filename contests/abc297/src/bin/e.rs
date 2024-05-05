//           E - Kth Takoyaki Set          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc297/tasks/abc297_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use superslice::Ext;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 解説
/// - ダイクストラ法
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N]
    }

    // ダイクストラ法
    let mut dist = vec![INF; K + 1];

    let mut pq = BinaryHeap::new();
    pq.push(Reverse(0));

    for k in 0..=K {
        while k > 0 && pq.peek().unwrap().0 == dist[k - 1] {
            pq.pop();
        }
        // k番目の値を確定させる
        dist[k] = pq.pop().unwrap().0;
        // 新たな最小値を探索する
        for &a in &A {
            pq.push(Reverse(dist[k] + a));
        }
    }

    println!("{}", dist[K]);
}
