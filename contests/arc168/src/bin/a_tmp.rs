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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use std::{
    fmt::Debug,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
};
/// # Monoid
/// - モノイド
pub trait Monoid {
    /// 値の型
    type Val: Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}
/// モノイドに対する逆元の実装
pub trait InversableMonoid: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}
/// モノイドに対する順序の実装
pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
    fn le(left: &Self::Val, right: &Self::Val) -> bool;
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
    /// BITの初期化を行う
    /// - `n`: 列の長さ
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::E; n + 1],
        }
    }
    /// 一点加算を行う
    /// - `i`: 加算を行うインデックス（`0-indexed`）
    /// - `x`: 加算する値
    pub fn add(&mut self, mut i: usize, x: T::Val) {
        i += 1;
        while i <= self.size {
            self.arr[i] = T::op(&self.arr[i], &x);
            i += Self::lsb(i);
        }
    }
    /// 先頭からの和を求める
    /// - `i`: 区間`[0,i)`に対しての総和（`0-indexed`）
    pub fn prefix_sum(&self, mut i: usize) -> T::Val {
        let mut res = T::E;
        while i != 0 {
            res = T::op(&res, &self.arr[i]);
            i -= Self::lsb(i);
        }
        res
    }
}
impl<T: InversableMonoid> BIT<T> {
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
    /// 任意の区間の和を求める
    /// - `range`: 区間を表すRangeオブジェクト
    fn sum<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
        if let Some((i, j)) = self.parse_range(range) {
            T::op(&self.prefix_sum(j), &T::inv(&self.prefix_sum(i)))
        } else {
            T::E
        }
    }
}
impl<T: Monoid> From<&Vec<T::Val>> for BIT<T> {
    /// ベクターの参照からBITを作成
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
    /// `lower_bound`/`upper_bound`を共通化した実装
    fn binary_search<F>(&self, w: T::Val, compare: F) -> usize
    where
        F: Fn(&T::Val, &T::Val) -> bool,
    {
        let mut sum = T::E;
        let mut idx = 0;
        let mut d = self.size.next_power_of_two() / 2;
        while d != 0 {
            if idx + d <= self.size {
                let nxt = T::op(&sum, &self.arr[idx + d]);
                if compare(&nxt, &w) {
                    sum = nxt;
                    idx += d;
                }
            }
            d >>= 1;
        }
        idx
    }
    /// `a_0 + a_1 + ... + a_i >= w`となる最小の`i`を求める
    pub fn lower_bound(&self, w: T::Val) -> usize {
        self.binary_search(w, T::lt)
    }
    /// `a_0 + a_1 + ... + a_i > w`となる最小の`i`を求める
    pub fn upper_bound(&self, w: T::Val) -> usize {
        self.binary_search(w, T::le)
    }
}
impl<T: InversableMonoid> Debug for BIT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BIT {{ [")?;
        for i in 0..self.size - 1 {
            write!(f, "{:?}, ", self.sum(i..i + 1))?;
        }
        write!(f, "{:?}] }}", self.sum(self.size - 1..self.size))
    }
}
pub mod Alg {
    use super::{InversableMonoid, Monoid, OrderedMonoid};
    #[derive(Debug)]
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
    impl InversableMonoid for Add {
        fn inv(val: &Self::Val) -> Self::Val {
            -val
        }
    }
    impl OrderedMonoid for Add {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
        fn le(left: &Self::Val, right: &Self::Val) -> bool {
            left <= right
        }
    }
    #[derive(Debug)]
    pub struct Mul;
    impl Monoid for Mul {
        type Val = isize;
        const E: Self::Val = 1;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
    #[derive(Debug)]
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left ^ right
        }
    }
    impl InversableMonoid for Xor {
        fn inv(val: &Self::Val) -> Self::Val {
            *val
        }
    }
}

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

fn main() {
    input! {
        N: usize,
        S: String,
    }

    // 数列の構築
    let mut arr = vec![0];

    for c in S.chars() {
        let last = arr.last().unwrap();
        if c == '<' {
            arr.push(last + 1);
        } else {
            arr.push(last - 1);
        }
    }

    // 最小値，最大値
    let mut mini = isize::MAX;
    let mut maxi = isize::MIN;
    for &a in &arr {
        chmin! {
            mini,
            a
        };
        chmax! {
            maxi,
            a
        };
    }

    // 0以上に置き換え
    maxi -= mini;

    for i in 0..N {
        arr[i] -= mini;
    }

    debug!(mini, maxi);
    debug!(arr);

    // 転倒数を求める
    let mut bit = BIT::<Alg::Add>::new(maxi as usize + 1);

    let mut inv = 0;

    for (i, &a) in arr.iter().enumerate() {
        inv += i - bit.prefix_sum(a as usize + 1) as usize;
        bit.add(a as usize, 1);
    }

    println!("{}", inv);
}
