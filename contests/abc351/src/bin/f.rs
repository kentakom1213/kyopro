#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::bit::{Alg::Add, BIT};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // 愚直解
    if cfg!(debug_assertions) {
        let mut D = vec![vec![0; N]; N];
        let mut sum = 0;
        for i in 0..N {
            for j in i + 1..N {
                if A[i] < A[j] {
                    D[i][j] = A[j] - A[i];
                    sum += A[j] - A[i];
                }
            }
        }
        debug2D!(D);
        debug!(sum);
    }

    // ソート
    let AI = A.iter().cloned().zip(0..).sorted().collect_vec();

    debug!(AI);

    // 平面走査
    let mut ans = 0;

    // 自分の前にある数の数
    let mut seg1 = BIT::<Add>::new(N);
    // 自分の前にある数の合計
    let mut seg2 = BIT::<Add>::new(N);

    for &(a, i) in &AI {
        // 自分の前にある数の数
        let cnt = seg1.prefix_sum(i);
        // 自分の前にある数の合計
        let sum = seg2.prefix_sum(i);

        ans += cnt * a - sum;

        // 設定
        seg1.add(i, 1);
        seg2.add(i, a);
    }

    println!("{ans}");
}

mod bit {
    //! BinaryIndexedTree / FenwickTree
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
        pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
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
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
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
}
