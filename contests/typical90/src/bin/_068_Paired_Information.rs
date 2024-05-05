//       068 - Paired Information（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bp
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use std::fmt;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};

/// # Monoid
pub trait Monoid {
    type Val: fmt::Debug + Clone + PartialEq;
    const E: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

/// # SegmentTree (Monoid)
pub struct SegmentTree<T: Monoid> {
    pub len: usize,
    offset: usize,
    data: Vec<T::Val>,
}

impl<T: Monoid> Index<usize> for SegmentTree<T> {
    type Output = T::Val;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<T: Monoid> SegmentTree<T> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.len,
            Excluded(&v) => v,
            Included(&v) => v - 1,
        };
        assert!(start <= end);
        (start, end)
    }

    /// ## new
    /// セグメント木を初期化する
    pub fn new(n: usize) -> Self {
        let len = n.next_power_of_two();

        Self {
            len,
            offset: len,
            data: vec![T::E; len << 1],
        }
    }

    fn update(&mut self, index: usize, value: T::Val) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = T::op(&self.data[lch], &self.data[lch + 1]);
        }
    }

    /// ## get_mut
    /// - 可変な参照を返す
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, T>> {
        if i < self.len {
            let default = self.index(i).clone();
            Some(ValMut {
                segtree: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
        let (start, end) = self.parse_range(range);

        // 全体の値を取得
        if (start, end) == (0, self.len) {
            return self.data[1].clone();
        }

        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (T::E, T::E);

        while l < r {
            if l & 1 == 1 {
                res_l = T::op(&res_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = T::op(&self.data[r], &res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        T::op(&res_l, &res_r)
    }
}

impl<T: Monoid> From<&Vec<T::Val>> for SegmentTree<T> {
    fn from(src: &Vec<T::Val>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = T::op(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }
}

pub struct ValMut<'a, T: 'a + Monoid> {
    segtree: &'a mut SegmentTree<T>,
    idx: usize,
    new_val: T::Val,
}

impl<T: Monoid> fmt::Debug for ValMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut")
            .field(&self.segtree.index(self.idx))
            .finish()
    }
}

impl<T: Monoid> Drop for ValMut<'_, T> {
    fn drop(&mut self) {
        self.segtree.update(self.idx, self.new_val.clone());
    }
}

impl<T: Monoid> Deref for ValMut<'_, T> {
    type Target = T::Val;
    fn deref(&self) -> &Self::Target {
        &self.segtree[self.idx]
    }
}

impl<T: Monoid> DerefMut for ValMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

pub mod Alg {
    use super::Monoid;

    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
}

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        queries: [(usize, Usize1, Usize1, isize); Q],
    }

    // 値が確定している区間を管理
    let mut ranges: BTreeSet<usize> = (0..=N).collect();

    // 区間の処理
    let mut seg = SegmentTree::<Alg::Add>::new(N - 1);

    // クエリの処理
    for &(t, x, y, v) in &queries {
        if t == 0 {
            // 区間のマージ（区間から右側を削除
            ranges.remove(&y);
            // 区間に追加
            if x % 2 == 0 {
                *seg.get_mut(x).unwrap() = v;
            } else {
                *seg.get_mut(x).unwrap() = -v;
            }
        } else {
            let (x_, y_) = if x > y { (y, x) } else { (x, y) }; // x <= yに修正
            if *ranges.range(..=y_).next_back().unwrap() <= x_ {
                // 区間を計算できる場合
                let mut sum = seg.get_range(x_..y_);
                if (x, y) != (x_, y_) {
                    sum *= -1;
                }
                let ans = match (x % 2, y % 2) {
                    (0, 0) => -(sum - v),
                    (0, 1) => sum - v,
                    (1, 0) => -(sum + v),
                    (1, 1) => sum + v,
                    _ => unreachable!(),
                };
                println!("{}", ans);
            } else {
                println!("Ambiguous");
            }
        }
    }
}
