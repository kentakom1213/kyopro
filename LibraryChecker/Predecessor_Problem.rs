// https://judge.yosupo.jp/problem/predecessor_problem

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

// main
fn main() {
    input! {
        Q: usize,
        T: String,
        queries: [(usize, usize); Q],
    }

    let mut bit = BIT::build(
        &T.chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>(),
    );

    for &(c, k) in &queries {
        match c {
            0 => {
                if bit.sum(k, k+1) == 0 {
                    bit.add(k, 1);
                }
            },
            1 => {
                if bit.sum(k, k+1) == 1 {
                    bit.add(k, -1);
                }
            },
            2 => {
                let is_contain = bit.sum(k, k+1) == 1;
                if is_contain {
                    println!("1");
                } else {
                    println!("0");
                }
            },
            3 => {
                let idx = bit.lower_bound(k as isize);
                println!("{}", idx);
            },
            _ => {
                println!("-1");
            }
        }
    }
}
