#![allow(non_snake_case)]

use itertools::Itertools;
use monoid::examples::Max;
use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        N: usize,
        mut P: [usize; N]
    }

    let mut D = vec![usize::MAX; N];

    // j < i && P[j] < P[i]
    calc(&P, &mut D);
    debug!(D);

    // j < i && P[j] > P[i]
    // P[i] を反転させる
    P.iter_mut().for_each(|x| *x = N + 1 - *x);
    calc(&P, &mut D);
    debug!(D);

    // j > i && P[j] > P[i]
    P.reverse();
    D.reverse();
    calc(&P, &mut D);
    debug!(D);

    // j > i && P[j] < P[i]
    P.iter_mut().for_each(|x| *x = N + 1 - *x);
    calc(&P, &mut D);
    debug!(D);

    // Dが反転していることに注意
    println!("{} ", D.iter().rev().join(" "));
}

/// 全ての(i,j)(i!=j)について、
/// chmin(
///     D[i],
///     P[i] + i - max_(j < i && P[j] < P[i])(P[j] + j)
/// );
/// を求める
fn calc(P: &[usize], D: &mut [usize]) {
    let mut seg = SegmentTree::<Max>::new(P.len());

    for i in 0..P.len() {
        if let Some(val) = seg.get_range(..P[i]) {
            chmin! {
                D[i],
                P[i] + i - val
            };
        }
        *seg.get_mut(P[i] - 1).unwrap() = Some(P[i] + i);
    }
}

mod macro_chmin {
    #![allow(dead_code)]
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

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}

mod affine1d {
    #![allow(dead_code)]
    //! 1次元Affine変換
    use std::ops::{Add, Mul};
    pub trait RingId {
        const ZERO: Self;
        const ONE: Self;
    }
    impl RingId for usize {
        const ZERO: Self = 0;
        const ONE: Self = 1;
    }
    impl RingId for isize {
        const ZERO: Self = 0;
        const ONE: Self = 1;
    }
    impl RingId for f64 {
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
    }
    /// Affine変換を表す型
    pub type Affine<T> = (T, T);
    pub trait AffineTransform<T> {
        const I: Self;
        /// affine変換をマージする
        ///
        /// - `self.compose(rhs)`：`self(rhs(x))`
        fn compose(&self, rhs: &Self) -> Self;
        /// スカラ値xに対し，affine変換を適用する
        fn apply(&self, x: T) -> T;
        /// affine変換を累乗する
        fn pow(&self, p: usize) -> Self;
    }
    impl<T> AffineTransform<T> for Affine<T>
    where
        T: Add<Output = T> + Mul<Output = T> + RingId + Copy,
    {
        const I: Self = (T::ONE, T::ZERO);
        fn compose(&self, rhs: &Self) -> Self {
            let &(a1, b1) = rhs;
            let &(a2, b2) = self;
            //   a2 * (a1 * x + b1) + b2
            // = (a2 * a1) * x + (a2 * b1 + b2)
            (a2 * a1, a2 * b1 + b2)
        }
        fn apply(&self, x: T) -> T {
            let &(a, b) = self;
            a * x + b
        }
        fn pow(&self, mut p: usize) -> Self {
            // 繰り返し2乗法
            let &(a, b) = self;
            let mut tmp = [[a, b], [T::ZERO, T::ONE]];
            let mut res = [[T::ONE, T::ZERO], [T::ZERO, T::ONE]];
            while p > 0 {
                if p & 1 == 1 {
                    res = dot(&tmp, &res);
                }
                tmp = dot(&tmp, &tmp);
                p >>= 1;
            }
            (res[0][0], res[0][1])
        }
    }
    type M2x2<T> = [[T; 2]; 2];
    fn dot<T>(x: &M2x2<T>, y: &M2x2<T>) -> M2x2<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        let &[[x11, x12], [x21, x22]] = x;
        let &[[y11, y12], [y21, y22]] = y;
        [
            [x11 * y11 + x12 * y21, x11 * y12 + x12 * y22],
            [x21 * y11 + x22 * y21, x21 * y12 + x22 * y22],
        ]
    }
}

mod monoid {
    #![allow(dead_code)]
    //! モノイド
    use crate::affine1d::{Affine, AffineTransform};
    use std::fmt::Debug;
    /// モノイド
    pub trait Monoid {
        /// 元の型
        type Val: Debug + Clone + PartialEq;
        /// 単位元
        fn id() -> Self::Val;
        /// 演算
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
    }
    /// 各種モノイド
    pub mod examples {
        use super::*;
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            fn id() -> Self::Val {
                0
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            fn id() -> Self::Val {
                1
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            fn id() -> Self::Val {
                0
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = usize;
            fn id() -> Self::Val {
                Self::Val::MAX
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = Option<usize>;
            fn id() -> Self::Val {
                None
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            fn id() -> Self::Val {
                Self::Val::MAX
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                gcd(*left, *right)
            }
        }
        pub fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        /// アフィン変換（浮動小数点数）
        struct Affine1d;
        impl Monoid for Affine1d {
            type Val = Affine<f64>;
            fn id() -> Self::Val {
                (1.0, 0.0)
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left.compose(&right)
            }
        }
    }
}

mod segment_tree {
    #![allow(dead_code)]
    //! セグメント木
    use crate::monoid::Monoid;
    use std::fmt::{self, Debug};
    use std::ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, DerefMut, Index, RangeBounds,
    };
    /// # SegmentTree (Monoid)
    /// - 抽象化セグメント木
    pub struct SegmentTree<M: Monoid> {
        pub size: usize,
        offset: usize,
        data: Vec<M::Val>,
    }
    impl<M: Monoid> Index<usize> for SegmentTree<M> {
        type Output = M::Val;
        fn index(&self, idx: usize) -> &Self::Output {
            &self.data[self.offset + idx]
        }
    }
    impl<M: Monoid> SegmentTree<M> {
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
        /// セグメント木を初期化する
        pub fn new(n: usize) -> Self {
            let offset = n;
            Self {
                size: n,
                offset,
                data: vec![M::id(); offset << 1],
            }
        }
        pub fn update(&mut self, index: usize, value: M::Val) {
            let mut i = index + self.offset;
            self.data[i] = value;
            while i > 1 {
                i >>= 1;
                let lch = i << 1;
                self.data[i] = M::op(&self.data[lch], &self.data[lch + 1]);
            }
        }
        /// 可変な参照を返す
        pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, M>> {
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
        pub fn get_range<R: RangeBounds<usize> + Debug>(&self, range: R) -> M::Val {
            let Some((start, end)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            // 値の取得
            let mut l = self.offset + start;
            let mut r = self.offset + end;
            let (mut res_l, mut res_r) = (M::id(), M::id());
            while l < r {
                if l & 1 == 1 {
                    res_l = M::op(&res_l, &self.data[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    res_r = M::op(&self.data[r], &res_r);
                }
                l >>= 1;
                r >>= 1;
            }
            M::op(&res_l, &res_r)
        }
    }
    impl<M: Monoid> From<&Vec<M::Val>> for SegmentTree<M> {
        fn from(src: &Vec<M::Val>) -> Self {
            let mut seg = Self::new(src.len());
            for (i, v) in src.iter().enumerate() {
                seg.data[seg.offset + i] = v.clone();
            }
            for i in (0..seg.offset).rev() {
                let lch = i << 1;
                seg.data[i] = M::op(&seg.data[lch], &seg.data[lch + 1]);
            }
            seg
        }
    }
    impl<M: Monoid> Debug for SegmentTree<M> {
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
    pub struct ValMut<'a, M: 'a + Monoid> {
        segtree: &'a mut SegmentTree<M>,
        idx: usize,
        new_val: M::Val,
    }
    impl<M: Monoid> Debug for ValMut<'_, M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("ValMut").field(&self.new_val).finish()
        }
    }
    impl<M: Monoid> Drop for ValMut<'_, M> {
        fn drop(&mut self) {
            self.segtree.update(self.idx, self.new_val.clone());
        }
    }
    impl<M: Monoid> Deref for ValMut<'_, M> {
        type Target = M::Val;
        fn deref(&self) -> &Self::Target {
            &self.new_val
        }
    }
    impl<M: Monoid> DerefMut for ValMut<'_, M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.new_val
        }
    }
    impl<M> SegmentTree<M>
    where
        M: Monoid,
        M::Val: Debug,
    {
        /// セグ木を簡易的に表示する
        /// **サイズが2べきのときのみ**
        pub fn show(&self) {
            #![cfg(debug_assertions)]
            let mut i = 1;
            let mut w = 1;
            while i + w <= 2 * self.offset {
                eprintln!("{:?}", &self.data[i..i + w]);
                i += w;
                w <<= 1;
            }
            eprintln!();
        }
    }
}
