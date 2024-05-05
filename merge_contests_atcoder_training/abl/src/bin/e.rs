// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};
/// 作用付きモノイド
pub trait ExtMonoid {
    /// 要素のデータ型
    type X: Clone + PartialEq;
    /// 作用素のデータ型
    type M: Clone + PartialEq;
    /// 要素Xの単位元
    const IX: Self::X;
    /// 作用素Mの単位元
    const IM: Self::M;
    /// 要素同士の演算
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X;
    /// 要素に対する作用
    fn apply(x: &Self::X, y: &Self::M) -> Self::X;
    /// 作用素同士の演算
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M;
    /// 作用素の集約
    fn aggregate(x: &Self::M, p: usize) -> Self::M;
}
/// 遅延評価セグメント木
#[derive(Debug)]
pub struct LazySegmentTree<T: ExtMonoid> {
    pub size: usize,
    offset: usize,
    data: Vec<T::X>,
    lazy: Vec<T::M>,
}
impl<T: ExtMonoid> LazySegmentTree<T> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }
    /// 遅延評価セグメント木を初期化する
    /// - `n`: 配列サイズ
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
            size: n,
            offset,
            data: vec![T::IX; offset << 1],
            lazy: vec![T::IM; offset << 1],
        }
    }
    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        if self.lazy[idx] == T::IM {
            return;
        }
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] = T::operate_m(&self.lazy[idx * 2], &self.lazy[idx]);
            self.lazy[idx * 2 + 1] = T::operate_m(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
        }
        // 自身を更新
        self.data[idx] = T::apply(&self.data[idx], &T::aggregate(&self.lazy[idx], len));
        self.lazy[idx] = T::IM;
    }
    /// 区間に`val`を作用させる
    /// - `range`: `[left, right)`
    pub fn apply<R: RangeBounds<usize>>(&mut self, range: R, val: T::M) {
        if let Some((left, right)) = self.parse_range(range) {
            self.apply_inner(left, right, val, 0, self.offset, 1);
        }
    }
    fn apply_inner(
        &mut self,
        left: usize,
        right: usize,
        val: T::M,
        begin: usize,
        end: usize,
        idx: usize,
    ) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] = T::operate_m(&self.lazy[idx], &val);
            self.eval(idx, end - begin);
        }
        // 区間が重なるとき
        else if left < end && begin < right {
            let mid = (begin + end) / 2;
            // 左の子を更新
            self.apply_inner(left, right, val.clone(), begin, mid, idx * 2);
            // 右の子を更新
            self.apply_inner(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = T::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }
    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get<R: RangeBounds<usize>>(&mut self, range: R) -> T::X {
        if let Some((left, right)) = self.parse_range(range) {
            self.get_inner(left, right, 0, self.offset, 1)
        } else {
            T::IX
        }
    }
    fn get_inner(
        &mut self,
        left: usize,
        right: usize,
        begin: usize,
        end: usize,
        idx: usize,
    ) -> T::X {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を含まない
        if end <= left || right <= begin {
            T::IX
        }
        // 区間を包含する
        else if left <= begin && end <= right {
            self.data[idx].clone()
        }
        // 区間が重なる
        else {
            let mid = (begin + end) / 2;
            let l_val = self.get_inner(left, right, begin, mid, idx * 2);
            let r_val = self.get_inner(left, right, mid, end, idx * 2 + 1);
            T::operate_x(&l_val, &r_val)
        }
    }
}
impl<T: ExtMonoid> From<&Vec<T::X>> for LazySegmentTree<T> {
    fn from(src: &Vec<T::X>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = T::operate_x(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }
}

pub const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

#[derive(Debug)]
pub struct ModDigits;
impl ExtMonoid for ModDigits {
    type X = (usize, usize); // (値、区間の長さ)
    type M = Option<usize>;
    const IX: Self::X = (0, 0);
    const IM: Self::M = None;
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        // 桁を揃えて足す
        let &(xv, xd) = x;
        let &(yv, yd) = y;
        let v = (xv * TEN[yd] % MOD + yv) % MOD;
        let d = xd + yd;
        (v, d)
    }
    fn apply(x: &Self::X, y: &Self::M) -> Self::X {
        (y.unwrap() * ONE[x.1] % MOD, x.1)
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        *y
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        *x
    }
}

const SIZE: usize = 202020;

const TEN: [usize; SIZE] = {
    let mut arr = [1; SIZE];
    let mut i = 1;
    while i < SIZE {
        arr[i] = arr[i - 1] * 10 % MOD;
        i += 1;
    }
    arr
};

const ONE: [usize; SIZE] = {
    let mut arr = [1; SIZE];
    let mut i = 2;
    while i < SIZE {
        arr[i] = (arr[i - 1] * 10 + 1) % MOD;
        i += 1;
    }
    arr
};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        queries: [(Usize1, usize, usize); Q]
    }

    let mut seg = LazySegmentTree::<ModDigits>::from(&vec![(1, 1); N]);

    // debug!(&seg);

    for &(l, r, d) in &queries {
        // 更新
        seg.apply(l..r, Some(d));
        // 取得
        println!("{}", seg.get(..).0);
    }
}
