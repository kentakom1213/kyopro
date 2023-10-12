// https://atcoder.jp/contests/abc153/tasks/abc153_f

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
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

/// ## RSQandRAQ
/// - 区間加算
/// - 区間和
#[derive(Debug)]
pub struct RSQandRAQ;

impl Monoid for RSQandRAQ {
    type X = isize;
    type M = isize;
    const IX: Self::X = 0;
    const IM: Self::M = 0;
    fn fx(x: &Self::X, y: &Self::X) -> Self::X {
        x + y
    }
    fn fa(x: &Self::X, y: &Self::M) -> Self::X {
        x + y
    }
    fn fm(x: &Self::M, y: &Self::M) -> Self::M {
        x + y
    }
    fn fp(x: &Self::M, p: usize) -> Self::M {
        x * p as isize
    }
}

fn main() {
    input! {
        N: usize,
        D: usize,
        A: isize,
        monster: [(usize, isize); N],
    }

    // 座標圧縮
    let comp = monster.iter().map(|v| v.0).sorted().collect_vec();

    debug!(&comp);

    // 遅延セグ木を構築
    let mut seg = LazySegmentTree::<RSQandRAQ>::new(N);

    // 値の設定
    for &(x, h) in &monster {
        let i = comp.lower_bound(&x);
        seg.set_range(i, i + 1, h);
    }

    // 前から貪欲に体力を減らしていく
    let mut ans = 0;

    for (i, &x) in comp.iter().enumerate() {
        // 現在の残り体力
        let rem = seg.get_range(i, i + 1);
        // まだ体力が残っている場合
        if rem > 0 {
            let l = comp.lower_bound(&x);
            let r = comp.upper_bound(&(x + 2 * D));
            // 攻撃
            let cnt = (rem + A - 1) / A;
            let damage = A * cnt;
            debug!(l, r, cnt);
            seg.set_range(l, r, -damage);
            ans += cnt;
        }
    }

    println!("{ans}");
}
