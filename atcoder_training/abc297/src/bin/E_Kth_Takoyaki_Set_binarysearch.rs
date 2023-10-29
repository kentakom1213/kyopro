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
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use rustc_hash::FxHashSet;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 解説
/// - 2分探索で解く
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N]
    }
    A.sort();

    let mut ng = 0;
    let mut ok = 300_000_000_000_000;
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if lt_K(mid, &A, K) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

/// xが安い方からK番目の金額以上であるか
fn lt_K(x: usize, A: &[usize], K: usize) -> bool {
    let mut visited = FxHashSet::default();
    visited.insert(0);
    let mut st = vec![0];
    while !st.is_empty() && visited.len() <= K {
        let cur = st.pop().unwrap();
        for &a in A {
            let nxt = cur + a;
            if visited.contains(&nxt) { continue; }
            if nxt > x { break; }
            st.push(nxt);
            visited.insert(nxt);
        }
    }
    visited.len() > K
}
