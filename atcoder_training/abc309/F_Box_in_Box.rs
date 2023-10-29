//              F - Box in Box
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc309/tasks/abc309_f
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
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};
use std::{cmp::Reverse, fmt};

/// # Monoid
pub trait Monoid {
    type Val: fmt::Debug + Clone + PartialEq;
    const E: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

/// # SegmentTree (Monoid)
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
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.offset,
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
        let offset = n.next_power_of_two();

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

    /// ## get_mut
    /// - 可変な参照を返す
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

    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
        let parsed = self.parse_range(range);
        if parsed.is_none() {
            return T::E;
        }

        let (start, end) = parsed.unwrap();

        // 全体の値を取得
        if (start, end) == (0, self.offset) {
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

/// # 座標圧縮
#[derive(Debug)]
pub struct Compression<'a, T> {
    pub size: usize,
    pub sorted_array: Vec<&'a T>,
}

impl<'a, T: Ord> Compression<'a, T> {
    pub fn new(array: &'a [T]) -> Self {
        let mut comp: Vec<&T> = array.iter().collect();
        comp.sort();
        comp.dedup();
        Self {
            size: comp.len(),
            sorted_array: comp,
        }
    }

    /// 圧縮後の番号を返す
    pub fn idx(&self, val: &T) -> Option<usize> {
        let idx = self.sorted_array.lower_bound(&val);
        if self.sorted_array.get(idx) == Some(&val) {
            Some(idx)
        } else {
            None
        }
    }

    /// 圧縮前の要素を返す
    pub fn val(&self, idx: usize) -> Option<&&T> {
        self.sorted_array.get(idx)
    }
}

// main
fn main() {
    input! {
        N: usize,
        boxes: [(usize, usize, usize); N],
    }

    let boxes: Vec<(usize, usize, usize)> = boxes
        .iter()
        // 各要素をソートする
        .map(|&(x, y, z)| {
            let mut v = vec![x, y, z];
            v.sort();
            (v[0], v[1], v[2])
        })
        // (x, -y)で昇順にソート
        .sorted_by_key(|&(x, y, z)| (x, Reverse(y)))
        .collect();

    debug!(&boxes);

    // それぞれベクタに
    let mut xs = vec![];
    let mut ys = vec![];
    let mut zs = vec![];
    for &(x, y, z) in &boxes {
        xs.push(x);
        ys.push(y);
        zs.push(z);
    }

    // 座標圧縮
    let comp_y = Compression::new(&ys);

    // セグ木を用意
    // seg[l..r] := l <= y < rとなる箱のうち、zの最大値
    pub struct Min;
    impl Monoid for Min {
        type Val = usize;
        const E: Self::Val = std::usize::MAX;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }

    let mut seg = SegmentTree::<Min>::new(comp_y.size);

    // 順に探索
    for &(x, y, z) in &boxes {
        // yの値が真に小さい箱の中で、zが真に小さい箱は存在するか
        let y_idx = comp_y.idx(&y).unwrap();
        let z_mini = seg.get_range(..y_idx);

        if z_mini < z {
            debug!(z_mini, z);
            println!("Yes");
            return;
        }

        {
            // 保存
            let mut z_cur = seg.get_mut(y_idx).unwrap();

            if z < *z_cur {
                *z_cur = z;
            }
        }

        debug!(&seg);
    }

    println!("No");
}
