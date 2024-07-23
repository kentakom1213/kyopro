#![allow(non_snake_case)]

use crate::cp_library_rs::{
    debug, debug2D,
    number_theory::{modint::M998, num_traits::Zero},
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    // 便宜上，0 <= A[i] < M とする．

    // dp[i][a][b][c] := i個目まで見たとき，
    //     長さ1の増加部分列の末尾の最小値がa
    //     長さ2の増加部分列の末尾の最小値がb
    //     長さ3の増加部分列の末尾の最小値がc
    //     であるような数列の組合せの数
    //     （明らかに，a <= b <= c）
    let mut dp = vec![[[[M998::zero(); 11]; 11]; 11]; N + 1];

    dp[0][M][M][M] = 1.into();

    for i in 0..N {
        for a in 0..=M {
            for b in 0..=M {
                for c in 0..=M {
                    // 新しくxを追加するとき
                    for x in 0..M {
                        let cur = dp[i][a][b][c];

                        if x <= a {
                            // 長さ1のLISの末尾の最小値が増える
                            dp[i + 1][x][b][c] += cur;
                        } else if x <= b {
                            // 長さ2のLISの末尾の最小値が増える
                            dp[i + 1][a][x][c] += cur;
                        } else if x <= c {
                            // 長さ3のLISの末尾の最小値が増える
                            dp[i + 1][a][b][x] += cur;
                        } else {
                            // これ以上の数は追加できない
                            break;
                        }
                    }
                }
            }
        }
    }

    debug2D!(dp);

    let mut ans = M998::zero();

    for a in 0..M {
        for b in 0..M {
            for c in 0..M {
                debug!(dp[N][a][b][c]);
                ans += dp[N][a][b][c];
            }
        }
    }

    println!("{ans}");
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod number_theory {
        pub mod modint {
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
                use crate::cp_library_rs::number_theory::num_traits::{One, Zero};
                #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)] pub struct Modint<const MOD: usize>(pub usize);
                impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) }
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
                impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError;
                fn from_str(s: &str) -> Result<Self, Self::Err> { let chunk_size = 9; let mut chars = s.chars(); let mut chunk = chars.by_ref().take(chunk_size).collect::<String>(); let mut res = Modint::zero();
                while !chunk.is_empty() { res = res * Modint::new(10).pow(chunk.len()) + chunk.parse::<usize>()?; chunk = chars.by_ref().take(chunk_size).collect::<String>(); } Ok(res) } }
                // impl<const MOD: usize> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
                impl<const MOD: usize> Zero for Modint<MOD> { fn zero() -> Self { Modint(0) } }
                impl<const MOD: usize> One for Modint<MOD> { fn one() -> Self { Modint(1) } }
                pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
                impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
                impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
                impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
            }
        }
        pub mod num_traits {
            pub use traits::*;
            #[rustfmt::skip]
            mod traits {
                use std::ops::{Add, Mul};
                pub trait Zero: Sized + Add<Self, Output = Self> { fn zero() -> Self; fn is_zero(&self) -> bool where Self: PartialEq, { self == &Self::zero() } }
                pub trait One: Sized + Mul<Self, Output = Self> { fn one() -> Self; fn is_one(&self) -> bool where Self: PartialEq, { self == &Self::one() } }
                impl Zero for usize { fn zero() -> Self { 0 } }    
                impl One for usize { fn one() -> Self { 1 } }
                impl Zero for isize { fn zero() -> Self { 0 } }
                impl One for isize { fn one() -> Self { 1 } }
                impl Zero for f64 { fn zero() -> Self { 0.0 } }
                impl One for f64 { fn one() -> Self { 1.0 } }
            }
        }
    }
    pub mod debug {
        /// デバッグ用マクロ
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
    pub mod debug2D {
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
}
