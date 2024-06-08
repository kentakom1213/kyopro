#![allow(non_snake_case)]

use extmonoid::ExtMonoid;
use itertools::Itertools;
use modint::{Modint, M998};
use proconio::{input, marker::Usize1};

use crate::lazy_segment_tree::LazySegmentTree;

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [M998; N],
        B: [M998; N],
    }

    let mut seg = LazySegmentTree::<AddSumMod>::from(
        &A.iter()
            .zip(&B)
            .map(|(&a, &b)| (1, a, b, a * b))
            .collect_vec(),
    );

    for _ in 0..Q {
        input! {t: usize}

        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: M998,
                }
                seg.apply(l..r, (x.into(), 0.into()));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: M998,
                }
                seg.apply(l..r, (0.into(), x.into()));
            }
            3 => {
                input! {
                    l: Usize1,
                    r: usize
                }
                let res = seg.get(l..r).3;
                println!("{res}");
            }
            _ => (),
        }
    }
}

/// ## AddSum
/// - 区間加算
/// - 区間和
#[derive(Debug)]
pub struct AddSumMod;
impl ExtMonoid for AddSumMod {
    // (a, b, a*b)
    type X = (usize, M998, M998, M998);
    // (fa, fb)
    type M = (M998, M998);
    const IX: Self::X = (0, Modint(0), Modint(0), Modint(0));
    const IM: Self::M = (Modint(0), Modint(0));
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        let &(xn, xa, xb, xab) = x;
        let &(yn, ya, yb, yab) = y;
        (xn + yn, xa + ya, xb + yb, xab + yab)
    }
    fn apply(x: &Self::X, y: &Self::M) -> Self::X {
        let &(n, a, b, ab) = x;
        let &(fa, fb) = y;
        (
            n,
            a + fa * n,
            b + fb * n,
            ab + fb * a + fa * b + fa * fb * n,
        )
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        let &(xa, xb) = x;
        let &(ya, yb) = y;
        (xa + ya, xb + yb)
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        // let &(fa, fb) = x;
        // (fa * p, fb * p)
        *x
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

mod extmonoid {
    #![allow(dead_code)]
    //! 作用付きモノイド
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
    /// （遅延セグ木）作用付きモノイド
    pub mod examples {
        use super::ExtMonoid;
        /// ## AddSum
        /// - 区間加算
        /// - 区間和
        #[derive(Debug)]
        pub struct AddSum;
        impl ExtMonoid for AddSum {
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
        /// ## UpdateMin
        /// - 区間更新
        /// - 区間最小値
        #[derive(Debug)]
        pub struct UpdateMin;
        impl ExtMonoid for UpdateMin {
            type X = isize;
            type M = isize;
            const IM: Self::M = isize::MAX;
            const IX: Self::X = isize::MAX;
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
        /// ## UpdateMax
        /// - 区間更新
        /// - 区間最大値
        #[derive(Debug)]
        pub struct UpdateMax;
        impl ExtMonoid for UpdateMax {
            type X = isize;
            type M = isize;
            const IM: Self::M = isize::MIN;
            const IX: Self::X = isize::MIN;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x.max(y)
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
        /// ## AddMin
        /// - 区間加算
        /// - 区間最小値
        #[derive(Debug)]
        pub struct AddMin;
        impl ExtMonoid for AddMin {
            type X = isize;
            type M = isize;
            const IM: Self::M = 0;
            const IX: Self::X = 1 << 31;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x.min(y)
            }
            fn apply(x: &Self::X, y: &Self::M) -> Self::X {
                x + y
            }
            fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
                x + y
            }
            fn aggregate(x: &Self::M, _p: usize) -> Self::M {
                *x
            }
        }
        /// ## UpdateSum
        /// - 区間更新
        /// - 区間和取得
        #[derive(Debug)]
        pub struct UpdateSum;
        impl ExtMonoid for UpdateSum {
            type X = isize;
            type M = Option<isize>;
            const IX: Self::X = 0;
            const IM: Self::M = None;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                x + y
            }
            fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
                y.unwrap()
            }
            fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
                *y
            }
            fn aggregate(x: &Self::M, p: usize) -> Self::M {
                x.map(|x| x * p as isize)
            }
        }
    }
}

mod lazy_segment_tree {
    #![allow(dead_code)]
    //! 遅延評価セグメント木
    use crate::extmonoid::ExtMonoid;
    use core::fmt;
    use std::{
        fmt::Debug,
        ops::{
            Bound::{Excluded, Included, Unbounded},
            RangeBounds,
        },
    };
    /// 遅延評価セグメント木
    #[derive(Debug)]
    pub struct LazySegmentTree<M: ExtMonoid> {
        pub size: usize,
        offset: usize,
        data: Vec<M::X>,
        lazy: Vec<M::M>,
    }
    impl<M: ExtMonoid> LazySegmentTree<M> {
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
        /// 遅延評価セグメント木を初期化する
        /// - `n`: 配列サイズ
        pub fn new(n: usize) -> Self {
            let offset = n.next_power_of_two();
            Self {
                size: n,
                offset,
                data: vec![M::IX; offset << 1],
                lazy: vec![M::IM; offset << 1],
            }
        }
        /// 遅延値を評価
        fn eval(&mut self, idx: usize, len: usize) {
            if self.lazy[idx] == M::IM {
                return;
            }
            // 葉でなければ子に伝搬
            if idx < self.offset {
                self.lazy[idx * 2] = M::operate_m(&self.lazy[idx * 2], &self.lazy[idx]);
                self.lazy[idx * 2 + 1] = M::operate_m(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
            }
            // 自身を更新
            self.data[idx] = M::apply(&self.data[idx], &M::aggregate(&self.lazy[idx], len));
            self.lazy[idx] = M::IM;
        }
        /// 区間に`val`を作用させる
        /// - `range`: `[left, right)`
        pub fn apply<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R, val: M::M) {
            let Some((left, right)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            self.apply_inner(left, right, val, 0, self.offset, 1);
        }
        fn apply_inner(
            &mut self,
            left: usize,
            right: usize,
            val: M::M,
            begin: usize,
            end: usize,
            idx: usize,
        ) {
            // 遅延値を評価
            self.eval(idx, end - begin);
            // 区間を内包するとき
            if left <= begin && end <= right {
                self.lazy[idx] = M::operate_m(&self.lazy[idx], &val);
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
                self.data[idx] = M::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
            }
        }
        /// 区間を取得する
        /// - `range`: `[left, right)`
        pub fn get<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R) -> M::X {
            let Some((left, right)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            self.get_inner(left, right, 0, self.offset, 1)
        }
        fn get_inner(
            &mut self,
            left: usize,
            right: usize,
            begin: usize,
            end: usize,
            idx: usize,
        ) -> M::X {
            // 遅延値を評価
            self.eval(idx, end - begin);
            // 区間を含まない
            if end <= left || right <= begin {
                M::IX
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
                M::operate_x(&l_val, &r_val)
            }
        }
    }
    impl<M: ExtMonoid> From<&Vec<M::X>> for LazySegmentTree<M> {
        fn from(src: &Vec<M::X>) -> Self {
            let mut seg = Self::new(src.len());
            for (i, v) in src.iter().enumerate() {
                seg.data[seg.offset + i] = v.clone();
            }
            for i in (0..seg.offset).rev() {
                let lch = i << 1;
                seg.data[i] = M::operate_x(&seg.data[lch], &seg.data[lch + 1]);
            }
            seg
        }
    }
    impl<M> LazySegmentTree<M>
    where
        M: ExtMonoid,
        M::M: Debug,
        M::X: Debug,
    {
        pub fn show(&mut self) -> String {
            let mut res = "LazySegmentTree {{ [".to_string();
            for i in 0..self.size {
                if i + 1 < self.size {
                    res += &format!("{:?}, ", self.get(i..=i));
                } else {
                    res += &format!("{:?}", self.get(i..=i));
                }
            }
            res += "] }}";
            res
        }
    }
}
