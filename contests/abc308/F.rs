// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BTreeSet};

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

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
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
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
        if start <= end {
            Some((start, end))
        } else {
            None
        }
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
        let parsed = self.parse_range(range);
        if parsed.is_none() {
            return T::E;
        }

        let (start, end) = parsed.unwrap();

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

    pub struct Max;
    impl Monoid for Max {
        type Val = ((usize, usize), usize);
        const E: Self::Val = ((0, 0), 0);
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }
}

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        P: [usize; N],
        L: [usize; M],
        D: [usize; M],
    }

    // 商品をソート
    let items = P.iter().cloned().sorted().collect_vec();

    // // クーポンの有効範囲でソート
    // let coupons = L
    //     .iter()
    //     .cloned()
    //     .zip(D.iter().cloned())
    //     .collect::<BTreeSet<(usize, usize)>>();

    // クーポンをソートして座標圧縮
    let coupen_sorted = L.iter().cloned().sorted().collect_vec();

    debug!(&coupen_sorted);

    let to_seg = D
        .iter()
        .cloned()
        .zip(0..N)
        .sorted_by_key(|&(_, i)| Reverse(items[i]))
        .zip(0..N)
        .collect_vec();

    // Lがx以下の要素で、Dが最大のものを取得
    // セグメント木で実現
    let mut seg = SegmentTree::<Alg::Max>::from(&to_seg);

    debug!(&seg.data[seg.offset..]);

    // 解の取得
    let mut ans = 0;

    for &p in &items {
        let idx = coupen_sorted.upper_bound(&p);

        // p円以下のクーポンで、最も価値が大きいもの
        let ((d, i), seg_i) = seg.get_range(..idx);

        debug!(p, d, i, idx);

        ans += p;

        if d != 0 {
            let l = L[i];

            // クーポンを使用
            ans -= d;

            // クーポンの削除
            *seg.get_mut(seg_i).unwrap() = ((0, 0), 0);
        }

        debug!(&seg.data[seg.offset..]);
    }

    println!("{}", ans);
}
