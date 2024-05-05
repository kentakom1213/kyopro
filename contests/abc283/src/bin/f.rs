#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        P: [isize; N],
    }

    // A[i] := P[i] + i
    let A = (1..).zip(&P).map(|(i, &p)| p + i).collect_vec();
    // B[i] := P[i] - i
    let B = (1..).zip(&P).map(|(i, &p)| p - i).collect_vec();


}

fn calc(n: usize, P: &[isize], D: &mut [isize]) {
    let mut seg = SegmentTree::<Alg::Max>::new(n);
    for i in 0..n {
        chmin! {
            D[i],
            P[i] + i as isize - seg.get_range(..P[i] as usize)
        };
        *seg.get_mut(P[i] as usize).unwrap() = P[i] + 1;
    }
}

use std::fmt;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};
/// モノイド
pub trait Monoid {
    /// 元の型
    type Val: fmt::Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}
/// # SegmentTree (Monoid)
/// - 抽象化セグメント木
pub struct SegmentTree<T: Monoid> {
    pub size: usize,
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
    fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        };
        if start <= end && end <= self.size {
            Some((start, end))
        } else {
            None
        }
    }
    /// ## new
    /// セグメント木を初期化する
    pub fn new(n: usize) -> Self {
        let offset = n;
        Self {
            size: n,
            offset,
            data: vec![T::E; offset << 1],
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
    /// 可変な参照を返す
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, T>> {
        if i < self.offset {
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
    /// 区間`range`の集約を行う
    pub fn get_range<R: RangeBounds<usize> + fmt::Debug>(&self, range: R) -> T::Val {
        let Some((start, end)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
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
impl<T: Monoid> std::fmt::Debug for SegmentTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree {{ [").ok();
        for i in 0..self.size {
            if i + 1 < self.size {
                write!(f, "{:?}, ", self.data[self.offset + i]).ok();
            } else {
                write!(f, "{:?}", self.data[self.offset + i]).ok();
            }
        }
        write!(f, "] }}")
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
/// さまざまな代数的構造
pub mod Alg {
    use super::Monoid;
    /// 最大値
    pub struct Max;
    impl Monoid for Max {
        type Val = isize;
        const E: Self::Val = -1001001001001001001;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }
}
