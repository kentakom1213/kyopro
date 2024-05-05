//          E - Wrapping Chocolate
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc245/tasks/abc245_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug)]
pub struct MultiSet<T> {
    pub map: BTreeMap<T, usize>,
    len: usize,
}

impl<T> MultiSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        MultiSet {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, x: T) {
        *self.map.entry(x).or_insert(0) += 1;
        self.len += 1;
    }

    pub fn remove(&mut self, x: &T) {
        if let Some(v) = self.map.get_mut(&x) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(&x);
            }
            self.len -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}


/// ## 方針
/// 1. 最も高さが低い箱を選ぶ
/// 2. その箱に入るチョコレートの中で最も幅が広いものを選ぶ
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; M],
        D: [usize; M],
    }
    // チョコレート
    let chocos = A.into_iter().zip(B.into_iter()).sorted().collect_vec();

    // 高さ順にソート
    let boxes = C.into_iter().zip(D.into_iter()).sorted().collect_vec();

    // 多重集合
    let mut S = MultiSet::new();
    let mut res = 0;
    let mut i = 0;

    for j in 0..M {
        // A[i] <= C[j] となる範囲のiをmultisetに移動
        while i < N && chocos[i].0 <= boxes[j].0 {
            S.insert(chocos[i].1);
            i += 1;
        }

        // その範囲での最大幅を取得
        let max_w = S.map.range(..= boxes[j].1).next_back();

        if let Some((&v, _)) = max_w {
            res += 1;
            S.remove(&v);
        }
    }

    if res == N {
        println!("Yes");
    } else {
        println!("No");
    }
}
