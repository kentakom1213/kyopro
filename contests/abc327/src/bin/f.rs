// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
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
    pub fn apply_range<R: RangeBounds<usize>>(&mut self, range: R, val: T::M) {
        if let Some((left, right)) = self.parse_range(range) {
            self.apply_range_inner(left, right, val, 0, self.offset, 1);
        }
    }
    fn apply_range_inner(
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
            self.apply_range_inner(left, right, val.clone(), begin, mid, idx * 2);
            // 右の子を更新
            self.apply_range_inner(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = T::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }
    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get_range<R: RangeBounds<usize>>(&mut self, range: R) -> T::X {
        if let Some((left, right)) = self.parse_range(range) {
            self.get_range_inner(left, right, 0, self.offset, 1)
        } else {
            T::IX
        }
    }
    fn get_range_inner(
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
            let l_val = self.get_range_inner(left, right, begin, mid, idx * 2);
            let r_val = self.get_range_inner(left, right, mid, end, idx * 2 + 1);
            T::operate_x(&l_val, &r_val)
        }
    }
}
pub mod Alg {
    use crate::INF;

    use super::ExtMonoid;
    /// ## RSQandRAQ
    /// - 区間加算
    /// - 区間和
    #[derive(Debug)]
    pub struct RSQandRAQ;
    impl ExtMonoid for RSQandRAQ {
        type X = isize;
        type M = isize;
        const IX: Self::X = 0;
        const IM: Self::M = 0;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            x + y
        }
        fn apply(x: &Self::X, y: &Self::M) -> Self::X {
            x + y
        }
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
            x + y
        }
        fn aggregate(x: &Self::M, p: usize) -> Self::M {
            x * p as isize
        }
    }
    /// ## RMQandRUQ
    /// - 区間更新
    /// - 区間最小値
    #[derive(Debug)]
    pub struct RMQandRUQ;
    impl ExtMonoid for RMQandRUQ {
        type X = isize;
        type M = isize;
        const IM: Self::M = (1 << 31) - 1;
        const IX: Self::X = (1 << 31) - 1;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            *x.min(y)
        }
        fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
            *y
        }
        fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
            *y
        }
        fn aggregate(x: &Self::M, _p: usize) -> Self::M {
            *x
        }
    }

    /// ## RMQandRUQ
    /// - 区間更新
    /// - 区間最小値
    #[derive(Debug)]
    pub struct RMQandRAQ;
    impl ExtMonoid for RMQandRAQ {
        type X = isize;
        type M = isize;
        const IM: Self::M = 0;
        const IX: Self::X = 0;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            *x.max(y)
        }
        fn apply(x: &Self::X, y: &Self::M) -> Self::X {
            x + y
        }
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
            x + y
        }
        fn aggregate(x: &Self::M, p: usize) -> Self::M {
            *x
        }
    }
}

const WIDTH: usize = 20;

fn main() {
    input! {
        N: usize,
        D: usize,
        W: usize,
        mut apples: [(usize, usize); N],
    }

    // りんごを時刻でソート
    apples.sort();

    debug!(&apples);

    // 尺取り法
    let mut deq = VecDeque::new(); // 時間について

    // num_x[i] := i <= x <= i + W となるようなりんごの個数
    let mut seg = LazySegmentTree::<Alg::RMQandRAQ>::new(WIDTH);

    let mut ans = 0;

    for i in 0..N {
        // りんごを追加
        let (t, x) = apples[i];
        deq.push_back((t, x));
        // (x - W, x + W)の範囲ではこのりんごを取れる
        let left = x.saturating_sub(W + 1);
        let right = WIDTH.min(x + W + 1);
        seg.apply_range(left..right, 1);
        debug!(left, right);

        // 区間から削除
        while !deq.is_empty() && deq.front().unwrap().0 + D < t {
            let (dt, dx) = deq.pop_front().unwrap();
            // (x - W, x + W)の範囲ではこのりんごを取れる
            let left = dx.saturating_sub(W + 1);
            let right = WIDTH.min(dx + W + 1);
            seg.apply_range(left..right, -1);
        }

        debug!(seg.get_range(..));
        debug!(&deq);

        if cfg!(debug_assertions) {
            for i in 0..WIDTH {
                eprint!("{} ", seg.get_range(i..i+1));
            }
            eprintln!();
        }

        // 区間最大値
        chmax! {
            ans,
            seg.get_range(..)
        };
    }

    println!("{}", ans);
}
