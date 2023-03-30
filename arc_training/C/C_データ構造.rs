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

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
pub struct BIT<T> {
    pub size: usize,
    arr: Vec<T>,
}

impl<T> BIT<T>
where
    T: Copy + Clone + Zero + Add + AddAssign + Sub<Output = T> + SubAssign + PartialOrd,
{
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::zero(); n + 1],
        }
    }

    pub fn build(src: &[T]) -> Self {
        let size = src.len();
        let mut arr = vec![T::zero(); size + 1];
        for i in 1..=size {
            let x = src[i - 1];
            arr[i] += x;
            let tmp = arr[i];
            let j = i + (i & i.wrapping_neg());
            if j < size + 1 {
                arr[j] += tmp;
            }
        }
        Self { size, arr }
    }

    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T {
        let mut idx = T::zero();
        while i != 0 {
            idx += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        idx
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        let to_l = self.prefix_sum(l);
        let to_r = self.prefix_sum(r);
        to_r - to_l
    }

    /// ## lower_bound
    /// - `0..x`の和が`w`以上になる最小の`x`を求める
    /// - 0-indexed
    pub fn lower_bound(&self, w: T) -> usize {
        let mut sum = T::zero();
        let mut idx = 0;
        let depth = {
            let mut d = 1;
            while d * 2 <= self.size {
                d *= 2;
            }
            d
        };
        for i in (0..=depth).rev() {
            let k = idx + (1 << i);
            if k <= self.size && sum + self.arr[k] < w {
                sum += self.arr[k];
                idx += 1 << i;
            }
        }
        idx
    }
}

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
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

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v < self[mid] {
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
    let mut bit = BIT::<isize>::new(comp.len());

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
