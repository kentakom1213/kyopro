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

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

use crate::{
    bit::{Alg::Add, BIT},
    coordinate_compression::Compression,
    zigzag::zigzag,
};

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [[usize; N]; N]
    }

    // Aを座標圧縮
    let comp = A
        .iter()
        .flat_map(|row| row.iter())
        .sorted()
        .cloned()
        .dedup()
        .collect_vec();

    debug!(comp);

    // BITでsetを構築
    let mut bit = BIT::<Add>::new(comp.len());

    // 最初のKxK領域の取得
    for i in 0..K {
        for j in 0..K {
            let idx = comp.lower_bound(&A[i][j]);
            bit.add(idx, 1);
        }
    }

    // 中央値を計算する
    let med = |bit: &BIT<Add>| -> usize {
        let m = (K * K + 1) / 2;
        let idx = bit.lower_bound(m as isize);
        comp[idx]
    };

    debug!(bit);

    // ジグザグに走査
    let mut ans = med(&bit);

    debug!(ans);

    let L = N - K + 1;
    let mut prev = (0, 0);

    for (top, left) in zigzag(L, L).skip(1) {
        // 領域の更新
        if prev.1 + 1 == left {
            debug!("right", top, left);
            // 右に移動した場合
            for i in 0..L {
                // 左側を削除
                let lidx = comp.lower_bound(&A[top + i][left - 1]);
                bit.add(lidx, -1);
                // 右側を追加
                let ridx = comp.lower_bound(&A[top + i][left + K - 1]);
                bit.add(ridx, 1);
            }
        } else if prev.0 + 1 == top {
            debug!("down", top, left);
            // 下に移動した場合
            for j in 0..L {
                // 左側を削除
                let lidx = comp.lower_bound(&A[top - 1][left + j]);
                bit.add(lidx, -1);
                // 右側を追加
                let ridx = comp.lower_bound(&A[top + K - 1][left + j]);
                bit.add(ridx, 1);
            }
        } else if left + 1 == prev.1 {
            debug!("left", top, left);
            // 左に移動した場合
            for i in 0..L {
                // 右側を削除
                let lidx = comp.lower_bound(&A[top + i][left + K]);
                bit.add(lidx, -1);
                // 左側を追加
                let ridx = comp.lower_bound(&A[top + i][left]);
                bit.add(ridx, 1);
            }
        }
        prev = (top, left);
        debug!(bit);
        // 領域の中央値を計算
        let tmp = med(&bit);
        debug!(tmp);

        // 更新
        chmin! {
            ans,
            tmp
        };
    }

    println!("{ans}");
}

mod macro_chmin {
    //! chminの実装
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
}

mod zigzag {
    //! 2次元グリッドをジグザグに走査する
    /// `H x W`領域をジグザグに操作する
    ///
    /// ↓3x3領域の例
    /// ```text
    /// 123
    /// 654
    /// 789
    /// ```
    pub fn zigzag(H: usize, W: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..H).zip(1..).step_by(2).flat_map(move |(t, b)| {
            (0..W)
                .map(move |j| (t, j))
                .chain((0..if b < H { W } else { 0 }).rev().map(move |j| (b, j)))
        })
    }
}

const INF: usize = 1001001001001001001;

mod coordinate_compression {
    //! 座標圧縮
    /// # 座標圧縮
    #[derive(Debug)]
    pub struct Compression<'a, T> {
        pub size: usize,
        pub sorted_array: Vec<&'a T>,
    }
    impl<'a, T: Ord> Compression<'a, T> {
        /// スライス`array`で配列を初期化する
        pub fn new(array: &'a [T]) -> Self {
            array.iter().collect()
        }
        /// 圧縮後の`val`の番号を返す
        pub fn idx(&self, val: &T) -> Option<usize> {
            let idx = self.sorted_array.binary_search(&val);
            if let Ok(idx) = idx {
                Some(idx)
            } else {
                None
            }
        }
        /// 圧縮前の要素`idx`を返す
        pub fn val(&self, idx: usize) -> Option<&T> {
            if let Some(&val) = self.sorted_array.get(idx) {
                Some(val)
            } else {
                None
            }
        }
    }
    impl<'a, T: Ord> FromIterator<&'a T> for Compression<'a, T> {
        fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
            let mut comp: Vec<&'a T> = iter.into_iter().collect();
            comp.sort();
            comp.dedup();
            Self {
                size: comp.len(),
                sorted_array: comp,
            }
        }
    }
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
}
