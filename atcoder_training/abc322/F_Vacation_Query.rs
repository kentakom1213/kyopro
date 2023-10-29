//            F - Vacation Query
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc322/tasks/abc322_f
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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## Monoid
pub trait Monoid {
    /// 要素のデータ型
    type X: Clone + PartialEq;
    /// 作用素のデータ型
    type M: Clone + PartialEq;

    /// 要素Xの単位元
    const IX: Self::X;
    /// 作用素Mの単位元
    const IM: Self::M;

    /// 要素同士の演算
    fn fx(x: &Self::X, y: &Self::X) -> Self::X;
    /// 要素に対する作用
    fn fa(x: &Self::X, y: &Self::M) -> Self::X;
    /// 作用素同士の演算
    fn fm(x: &Self::M, y: &Self::M) -> Self::M;
    /// 作用素の集約
    fn fp(x: &Self::M, p: usize) -> Self::M;
}

#[derive(Debug)]
pub struct LazySegmentTree<T: Monoid> {
    offset: usize,
    data: Vec<T::X>,
    lazy: Vec<T::M>,
}

impl<T: Monoid> LazySegmentTree<T> {
    /// 新規作成
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
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
            self.lazy[idx * 2] = T::fm(&self.lazy[idx * 2], &self.lazy[idx]);
            self.lazy[idx * 2 + 1] = T::fm(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
        }
        // 自身を更新
        self.data[idx] = T::fa(&self.data[idx], &T::fp(&self.lazy[idx], len));
        self.lazy[idx] = T::IM;
    }

    /// 区間加算
    /// - [left, right)
    pub fn set_range(&mut self, left: usize, right: usize, val: T::M) {
        self.set_range_inner(left, right, val, 0, self.offset, 1);
    }

    fn set_range_inner(
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
            self.lazy[idx] = T::fm(&self.lazy[idx], &val);
            self.eval(idx, end - begin);
        }
        // 区間が重なるとき
        else if left < end && begin < right {
            let mid = (begin + end) / 2;
            // 左の子を更新
            self.set_range_inner(left, right, val.clone(), begin, mid, idx * 2);
            // 右の子を更新
            self.set_range_inner(left, right, val.clone(), mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = T::fx(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }

    /// 区間取得
    /// - 再帰実装
    /// - [left, right)
    pub fn get_range(&mut self, left: usize, right: usize) -> T::X {
        self.get_range_inner(left, right, 0, self.offset, 1)
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
            T::fx(&l_val, &r_val)
        }
    }
}

struct Vacation;
impl Monoid for Vacation {
    type X = ();
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars,
        queries: [(usize, Usize1, usize); Q],
    }
}
