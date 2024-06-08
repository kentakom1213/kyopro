#![allow(non_snake_case)]

use itertools::Itertools;
use modint::Modint;
use monoid::Monoid;
use num_traits::Zero;
use proconio::input;

use crate::{
    modint::{Fp, M107},
    segment_tree::SegmentTree,
};

struct ABC;

impl Monoid for ABC {
    type Val = [usize; 4];
    fn id() -> Self::Val {
        [0; 4]
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        [0, 1, 2, 3].map(|i| left[i] + right[i])
    }
}

const M3: M107 = Modint(3);

fn main() {
    input! {
        S: String
    }

    let A = S
        .chars()
        .map(|c| match c {
            'A' => [1, 0, 0, 0],
            'B' => [0, 1, 0, 0],
            'C' => [0, 0, 1, 0],
            _ => [0, 0, 0, 1],
        })
        .collect_vec();

    let seg = SegmentTree::<ABC>::from(&A);

    // 'B'を全探索
    let mut ans = M107::zero();

    for (i, c) in S.chars().enumerate() {
        if c == 'B' || c == '?' {
            let [la, _, _, lq] = seg.get_range(..i);
            let [_, _, rc, rq] = seg.get_range(i + 1..);

            // 左側をAにする方法
            let L = M3.pow(lq) * la + (M3.pow(lq.saturating_sub(1))) * lq;
            let R = M3.pow(rq) * rc + (M3.pow(rq.saturating_sub(1))) * rq;

            ans += L * R;
        }
    }

    println!("{ans}");
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
            type Val = isize;
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
            type Val = isize;
            fn id() -> Self::Val {
                Self::Val::MIN
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

mod modint {
    #![allow(dead_code)]
    //! Modintの構造体
    pub use modint::*;
    pub type M998 = Modint<998244353>;
    pub type M107 = Modint<1000000007>;
    // 適当な素数
    pub type P1 = Modint<938472061>;
    pub type P2 = Modint<958472071>;
    #[rustfmt::skip]
    pub mod modint {
        fn sqrt(n: usize) -> usize { (n as f64).sqrt() as usize }
        use std::{fmt::{Debug, Display}, iter::{Sum, Product}, mem::replace, num::ParseIntError, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};
        use num_traits::{One, Zero};
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)] pub struct Modint<const MOD: usize>(pub usize);
        impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) }
        pub fn from_str(s: &str) -> Self { s.chars().fold(0.into(), |n, c| n * 10 + c.to_digit(10).unwrap() as usize) }
        pub fn from_isize(n: isize) -> Self { Self::new((MOD as isize + n % MOD as isize) as usize) }
        pub fn rational_reconstruction(&self) -> Option<(usize, usize)> { let N = sqrt(MOD / 2); let mut v = (MOD, 0); let mut w = (self.0, 1);
        while w.0 > N { let q = v.0.div_euclid(w.0); let z = (v.0 - q * w.0, v.1 + q * w.1); v = replace(&mut w, z); } (w.0 <= N && w.1 <= N).then_some(w) } }
        impl<const MOD: usize> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
        impl<const MOD: usize> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
        impl<const MOD: usize> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
        impl<const MOD: usize> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
        impl<const MOD: usize> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
        impl<const MOD: usize> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
        impl<const MOD: usize> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
        impl<const MOD: usize> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
        impl<const MOD: usize> DivAssign for Modint<MOD> { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }
        impl<const MOD: usize> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value) } }
        impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { self + Modint::new(rhs) } }
        impl<const MOD: usize> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
        impl<const MOD: usize> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
        impl<const MOD: usize> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
        impl<const MOD: usize> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
        impl<const MOD: usize> SubAssign<usize> for Modint<MOD> { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
        impl<const MOD: usize> MulAssign<usize> for Modint<MOD> { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
        impl<const MOD: usize> DivAssign<usize> for Modint<MOD> { fn div_assign(&mut self, rhs: usize) { *self /= Modint::new(rhs) } }
        impl<const MOD: usize> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
        impl<const MOD: usize> PartialEq<usize> for Modint<MOD> { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
        impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::from_str(s)) } }
        // impl<const MOD: usize> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
        impl<const MOD: usize> Zero for Modint<MOD> { fn zero() -> Self { Modint(0) } fn is_zero(&self) -> bool { self.0 == 0 } }
        impl<const MOD: usize> One for Modint<MOD> { fn one() -> Self { Modint(1) } }
        pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
        impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
        impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
        impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
    }
}
