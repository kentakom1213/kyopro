//           D - Prefix K-th Max
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc234/tasks/abc234_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num::CheckedDiv;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::{
    cmp::{max, min, Reverse},
    iter,
};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

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

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [usize; N],
    }

    let mut bit = BIT::<Alg::Add>::new(N);

    for i in 0..N {
        // 値をセット
        bit.add(P[i] - 1, 1);

        // デバッグ
        // for i in 0..N {
        //     print!("{} ", bit.prefix_sum(i+1) - bit.prefix_sum(i));
        // }
        // println!();

        if i >= K - 1 {
            // 小さい方から`i-K-1`番目を調べる
            let idx = bit.lower_bound((i + 1 - K + 1) as isize);
            println!("{}", idx + 1);
        }
    }
}
