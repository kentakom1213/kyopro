// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let MAX: usize = *A.iter().max().unwrap();

    // dp[i] := i番目までみたとき，すべてのペアの組合せに対するmaxの合計
    let mut dp = vec![Mod998::new(0); N + 1];

    // sum[i] := 集合の要素のうちi以下のものの総和
    let mut sum = BIT::<Alg::Add>::new(MAX + 1);

    // 集合に含まれる要素の個数を管理
    let mut set = BIT::<Alg::Add>::new(MAX + 1);

    for (i, &a) in A.iter().enumerate() {
        // 前の値を足す
        let tmp = dp[i];
        dp[i + 1] += tmp;
        // A[i]以下の要素が何個存在するか
        let le_a = set.prefix_sum(a + 1) as usize;
        // 2 * le_a + 1個のペアについては，そのmaxがA[i]になる
        dp[i + 1] += a * (2 * le_a + 1);
        // のこりの集合を加える
        let gt_a = sum.prefix_sum(MAX + 1) - sum.prefix_sum(a + 1);
        dp[i + 1] += 2 * gt_a as usize;
        // 集合に加算
        sum.add(a, a as isize);
        set.add(a, 1);
    }

    debug!(dp);
    debug!(sum);
    debug!(set);

    // 解の出力
    println!(
        "{}",
        dp[1..]
            .iter()
            .enumerate()
            .map(|(i, &x)| x / Mod998::new(i + 1).pow(2))
            .join("\n")
    );
}

#[rustfmt::skip]
pub mod modint {
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint<const MOD: usize>(pub usize);
    impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) } }
    impl<const MOD: usize> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl<const MOD: usize> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl<const MOD: usize> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl<const MOD: usize> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
    impl<const MOD: usize> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl<const MOD: usize> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl<const MOD: usize> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
    impl<const MOD: usize> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl<const MOD: usize> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value) } }
    impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
    impl<const MOD: usize> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
    impl<const MOD: usize> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
    impl<const MOD: usize> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
    impl<const MOD: usize> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
    impl<const MOD: usize> SubAssign<usize> for Modint<MOD> { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
    impl<const MOD: usize> MulAssign<usize> for Modint<MOD> { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
    impl<const MOD: usize> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
    impl<const MOD: usize> PartialEq<usize> for Modint<MOD> { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
    impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { usize::from_str(s).map(Modint::new) } }
    pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
    impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
    impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
}
use modint::*;
pub type Mod998 = Modint<998244353>;
pub type Mod1e9 = Modint<1000000007>;

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
