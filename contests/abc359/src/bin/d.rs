#![allow(non_snake_case)]

use itertools::Itertools;
use num_traits::{One, Zero};
use proconio::{input, marker::Chars};

use crate::modint::M998;

fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars
    }

    let is_pal = (0..1 << K)
        .map(|s| (0..K / 2).all(|i| (s >> i & 1) == (s >> (K - i - 1) & 1)))
        .collect_vec();

    // dp[i][j] := i文字目まで確定させたとき，K文字の回文を含まず，末尾K文字がjになっているようなものの総数
    //     0 => 'A', 1 => 'B'
    let mut dp = vec![[M998::zero(); 1024]; N + 1];

    // K文字目までを初期化
    for s in 0..1 << K {
        let mut isok = true;
        for i in 0..K {
            let c = if (s >> i & 1) == 0 { 'A' } else { 'B' };
            if S[i] != '?' && c != S[i] {
                isok = false;
                break;
            }
        }
        if isok && !is_pal[s] {
            dp[K][s] = 1.into();
        }
    }

    debug2D!(dp);

    for i in K..N {
        for s in 0..1 << K {
            let na = s >> 1;
            let nb = (s >> 1) | (1 << K - 1);

            // println!("i:{i}, s:{s:>10b}, na:{na:>10b}, nb:{nb:>10b}");

            let tmp = dp[i][s];

            if matches!(S[i], 'A' | '?') && !is_pal[na] {
                // println!("s:{s:>10b} -> na:{na:>10b}");
                dp[i + 1][na] += tmp;
            }

            if matches!(S[i], 'B' | '?') && !is_pal[nb] {
                // println!("s:{s:>10b} -> nb:{nb:>10b}");
                dp[i + 1][nb] += tmp;
            }
        }
    }

    debug2D!(dp);

    let ans = dp[N].iter().cloned().sum::<M998>();

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
