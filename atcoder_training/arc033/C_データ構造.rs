//                C - データ構造
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc033/tasks/arc033_3
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use num_traits::identities::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// # Monoid
pub trait Monoid {
    type Val: Clone + PartialEq;
    const E: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
}

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
pub struct BIT<T: Monoid> {
    pub size: usize,
    arr: Vec<T::Val>,
}

impl<T: Monoid> BIT<T> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::E; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: T::Val) {
        i += 1;
        while i <= self.size {
            self.arr[i] = T::op(&self.arr[i], &x);
            i += Self::lsb(i);
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T::Val {
        let mut res = T::E;
        while i != 0 {
            res = T::op(&res, &self.arr[i]);
            i -= Self::lsb(i);
        }
        res
    }
}

impl<T: Monoid> From<&Vec<T::Val>> for BIT<T> {
    fn from(src: &Vec<T::Val>) -> Self {
        let size = src.len();
        let mut arr = vec![T::E; size + 1];
        for i in 1..=size {
            let x = src[i - 1].clone();
            arr[i] = T::op(&arr[i], &x);
            let j = i + Self::lsb(i);
            if j < size + 1 {
                arr[j] = T::op(&arr[j], &arr[i].clone());
            }
        }
        Self { size, arr }
    }
}

impl<T: OrderedMonoid> BIT<T> {
    pub fn lower_bound(&self, w: T::Val) -> usize {
        let mut sum = T::E;
        let mut idx = 0;
        let mut d = self.size.next_power_of_two() / 2;
        while d != 0 {
            if idx + d <= self.size {
                let nxt = T::op(&sum, &self.arr[idx + d]);
                if T::lt(&nxt, &w) {
                    sum = nxt;
                    idx += d;
                }
            }
            d >>= 1;
        }
        idx
    }
}

mod Alg {
    use super::{Monoid, OrderedMonoid};

    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
    impl OrderedMonoid for Add {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
    }
}

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v <= self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

// main
fn main() {
    input! {
        Q: usize,
        queries: [(usize, usize); Q],
    }

    let comp = queries
        .iter()
        .map(|(_, v)| *v)
        .sorted()
        .dedup()
        .collect_vec();
    let mut bit = BIT::<Alg::Add>::new(comp.len());

    for &(t, x) in &queries {
        if t == 1 {
            let idx = comp.lower_bound(x);
            bit.add(idx, 1);
        } else {
            if bit.prefix_sum(comp.len()) < (x as isize) {
                println!("-1");
            }
            else {
                let idx = bit.lower_bound(x as isize);
                println!("{}", comp[idx]);
                bit.add(idx, -1);
            }
        }
    }
}
