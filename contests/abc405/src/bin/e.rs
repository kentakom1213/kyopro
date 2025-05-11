#![allow(non_snake_case)]

use crate::cp_library_rs::{
    debug,
    number_theory::{comb::Comb, modint::M998},
    utils::num_traits::Zero,
};
use proconio::input;

fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }

    let cmb = Comb::<M998>::new(20001001);

    let mut ans = M998::zero();

    // i := 最も左にある D の左にある文字(A,C)の個数
    for i in A..=A + C {
        // 残りの C+D-i 個の枠に D-1 個を入れる
        let chooseD = cmb.comb_with_rep(D - 1, A + C + 1 - i);

        // 左から i 番目までに B を入れる
        let chooseB = cmb.comb_with_rep(B, i + 1);

        let res = chooseD * chooseB;

        debug!(i, chooseD, chooseB);

        ans += res;
    }

    println!("{ans}");
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod number_theory {
        pub mod comb {
            //! 階乗を前計算する（Modint構造体に依存）
            use crate::cp_library_rs::{
                number_theory::modint::{M107, M998, MOD107, MOD998},
                utils::num_traits::{One, Zero},
            };
            use std::ops::{Add, Mul, Neg, Sub};
            /// 確率の値となりうる数
            pub trait NumProbability<Rhs = Self, Output = Self>:
                Clone
                + Copy
                + From<usize>
                + Neg<Output = Output>
                + Add<Rhs, Output = Output>
                + Sub<Rhs, Output = Output>
                + Mul<Rhs, Output = Output>
                + Mul<usize, Output = Output>
                + Zero
                + One
                + PartialEq
            {
                const MOD: usize;
            }
            impl NumProbability for M107 {
                const MOD: usize = MOD107 as usize;
            }
            impl NumProbability for M998 {
                const MOD: usize = MOD998 as usize;
            }
            /// 二項係数を高速に求める
            /// - 前計算:  $`O(N)`$
            /// - クエリ:  $`O(1)`$
            pub struct Comb<N: NumProbability> {
                fac: Vec<N>,
                finv: Vec<N>,
            }
            impl<N: NumProbability> Comb<N> {
                /// サイズ`max_size`で配列を初期化する
                pub fn new(max_size: usize) -> Self {
                    let mut fac = vec![N::one(); max_size];
                    let mut finv = vec![N::one(); max_size];
                    let mut inv = vec![N::one(); max_size];
                    for i in 2..max_size {
                        fac[i] = fac[i - 1] * i;
                        inv[i] = -N::from(N::MOD / i) * inv[N::MOD % i];
                        finv[i] = finv[i - 1] * inv[i];
                    }
                    Comb { fac, finv }
                }
                /// n個からr個選ぶ組合せ
                ///
                /// - 計算量：$`O(1)`$
                pub fn comb(&self, n: usize, r: usize) -> N {
                    if n < r {
                        return 0.into();
                    }
                    self.fac[n] * self.finv[r] * self.finv[n - r]
                }
                /// n個からr個を選び並べる順列
                ///
                /// - 計算量：$`O(1)`$
                pub fn perm(&self, n: usize, r: usize) -> N {
                    if n < r {
                        return 0.into();
                    }
                    self.fac[n] * self.finv[n - r]
                }
                /// 重複組合せ
                ///
                /// - n個の区別しない玉をr個の区別する箱に入れる組合せ
                /// - 計算量：$`O(1)`$
                pub fn comb_with_rep(&self, n: usize, r: usize) -> N {
                    self.comb(n + r - 1, n)
                }
            }
        }
        pub mod modint {
            //! Modintの構造体
            pub use modint::*;
            /// MOD用の定数：$`998244353`$
            pub const MOD998: u32 = 998244353;
            /// MOD用の定数：$`10^9 + 7`$
            pub const MOD107: u32 = 1000000007;
            pub type M998 = Modint<MOD998>;
            pub type M107 = Modint<MOD107>;
            // 適当な素数
            pub type P1 = Modint<938472061>;
            pub type P2 = Modint<958472071>;
            #[rustfmt::skip]
            pub mod modint {
                macro_rules! impl_ops {
                    ($t:ty, $op_trait:ident, $op_func:ident, $op:tt) => {
                        impl<const MOD: u32> $op_trait<$t> for Modint<MOD> { type Output = Self; fn $op_func(self, rhs: $t) -> Self { self $op Modint::from(rhs) } }
                    };
                    (assign, $t:ty, $op_trait:ident, $op_func:ident, $op:tt) => {
                        impl<const MOD: u32> $op_trait<$t> for Modint<MOD> { fn $op_func(&mut self, rhs: $t) { *self = *self $op Modint::from(rhs) } }
                    };
                    (partial_eq, $t:ty) => {
                        impl<const MOD: u32> PartialEq<$t> for Modint<MOD> { fn eq(&self, other: &$t) -> bool { self == &Modint::from(*other) } }
                    };
                    (all, $t:ty) => {
                        impl_ops!($t, Add, add, +);
                        impl_ops!($t, Sub, sub, -);
                        impl_ops!($t, Mul, mul, *);
                        impl_ops!($t, Div, div, /);
                        impl_ops!(assign, $t, AddAssign, add_assign, +);
                        impl_ops!(assign, $t, SubAssign, sub_assign, -);
                        impl_ops!(assign, $t, MulAssign, mul_assign, *);
                        impl_ops!(assign, $t, DivAssign, div_assign, /);
                        impl_ops!(partial_eq, $t);
                    };
                }
                fn sqrt(n: u32) -> usize { (n as f64).sqrt() as usize }
                use std::{fmt::{Debug, Display}, iter::{Product, Sum}, mem::replace, num::ParseIntError, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};
                use crate::cp_library_rs::utils::num_traits::{One, Zero};
                #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)] pub struct Modint<const MOD: u32>(pub u32);
                impl<const MOD: u32> Modint<MOD> { pub fn new(n: u32) -> Self { Self(if n < MOD { n } else { n % MOD }) }
                pub fn rational_reconstruction(&self) -> Option<(usize, usize)> { let N = sqrt(MOD / 2); let mut v = (MOD as usize, 0); let mut w = (self.0 as usize, 1);
                while w.0 > N { let q = v.0.div_euclid(w.0); let z = (v.0 - q * w.0, v.1 + q * w.1); v = replace(&mut w, z); } (w.0 <= N && w.1 <= N).then_some(w) } }
                impl<const MOD: u32> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
                impl<const MOD: u32> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
                impl<const MOD: u32> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
                impl<const MOD: u32> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32) } }
                impl<const MOD: u32> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
                impl<const MOD: u32> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
                impl<const MOD: u32> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
                impl<const MOD: u32> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
                impl<const MOD: u32> DivAssign for Modint<MOD> { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }
                impl<const MOD: u32> From<u32> for Modint<MOD> { fn from(value: u32) -> Self { Modint::new(value) } }
                impl<const MOD: u32> From<i32> for Modint<MOD> { fn from(value: i32) -> Self { Modint(value.rem_euclid(MOD as i32) as u32) } }
                impl<const MOD: u32> From<u64> for Modint<MOD> { fn from(value: u64) -> Self { Modint((value % MOD as u64) as u32) } }
                impl<const MOD: u32> From<i64> for Modint<MOD> { fn from(value: i64) -> Self { Modint(value.rem_euclid(MOD as i64) as u32) } }
                impl<const MOD: u32> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint((value % MOD as usize) as u32) } }
                impl<const MOD: u32> From<isize> for Modint<MOD> { fn from(value: isize) -> Self { Modint(value.rem_euclid(MOD as isize) as u32) } }
                // impl_ops!(all, u32);
                // impl_ops!(all, i32);
                // impl_ops!(all, u64);
                // impl_ops!(all, i64);
                impl_ops!(all, usize);
                // impl_ops!(all, isize);
                impl<const MOD: u32> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
                impl<const MOD: u32> FromStr for Modint<MOD> { type Err = ParseIntError;
                fn from_str(s: &str) -> Result<Self, Self::Err> { let chunk_size = 17; let mut chars = s.chars(); let mut chunk = chars.by_ref().take(chunk_size).collect::<String>(); let mut res = Modint::zero();
                while !chunk.is_empty() { res = res * Modint::new(10).pow(chunk.len() as u32) + chunk.parse::<usize>()?; chunk = chars.by_ref().take(chunk_size).collect::<String>(); } Ok(res) } }
                // impl<const MOD: u32> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
                impl<const MOD: u32> Zero for Modint<MOD> { fn zero() -> Self { Modint(0) } }
                impl<const MOD: u32> One for Modint<MOD> { fn one() -> Self { Modint(1) } }
                pub trait Fp { fn pow(&self, rhs: u32) -> Self; fn inv(&self) -> Self; }
                impl<const MOD: u32> Fp for Modint<MOD> { fn pow(&self, rhs: u32) -> Self { let (mut a, mut b) = (*self, rhs); let mut res = Modint::one(); while b > 0 { if b & 1 == 1 { res *= a; } a *= a; b >>= 1u32; } Modint::from(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
                impl<const MOD: u32> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
                impl<const MOD: u32> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
            }
        }
    }
    pub mod utils {
        pub mod num_traits {
            //! 数の構造
            #[rustfmt::skip] macro_rules! zero_impl { ($t:ty, $v:expr) => { impl Zero for $t { #[inline] fn zero() -> $t { $v } } } }
            #[rustfmt::skip] macro_rules! one_impl { ($t:ty, $v:expr) => { impl One for $t { #[inline] fn one() -> $t { $v } } } }
            #[rustfmt::skip] macro_rules! bounded_impl { ($t:ty, $min:expr, $max:expr) => { impl Bounded for $t { #[inline] fn min_value() -> $t { $min } #[inline] fn max_value() -> $t { $max } } } }
            pub use traits::*;
            #[rustfmt::skip]
            mod traits {
                use std::ops::{Add, Div, Mul, Rem, Sub};
                pub trait Zero: Sized + Add<Self, Output = Self> { fn zero() -> Self; fn is_zero(&self) -> bool where Self: PartialEq, { self == &Self::zero() } }
                zero_impl!(u32, 0);
                zero_impl!(usize, 0);
                zero_impl!(i32, 0);
                zero_impl!(isize, 0);
                zero_impl!(f32, 0.0);
                zero_impl!(f64, 0.0);
                
                pub trait One: Sized + Mul<Self, Output = Self> { fn one() -> Self; fn is_one(&self) -> bool where Self: PartialEq, { self == &Self::one() } }
                one_impl!(u32, 1);
                one_impl!(usize, 1);
                one_impl!(i32, 1);
                one_impl!(isize, 1);
                one_impl!(f32, 1.0);
                one_impl!(f64, 1.0);
                
                pub trait NumOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }
                impl NumOps for u32 {}
                impl NumOps for usize {}
                impl NumOps for i32 {}
                impl NumOps for isize {}
                impl NumOps for f32 {}
                impl NumOps for f64 {}
                pub trait Num: PartialEq + Zero + One + NumOps {}
                impl Num for u32 {}
                impl Num for usize {}
                impl Num for i32 {}
                impl Num for isize {}
                impl Num for f32 {}
                impl Num for f64 {}
                pub trait Bounded { fn min_value() -> Self; fn max_value() -> Self; }
                bounded_impl!(u32, std::u32::MIN, std::u32::MAX);
                bounded_impl!(usize, std::usize::MIN, std::usize::MAX);
                bounded_impl!(i32, std::i32::MIN, std::i32::MAX);
                bounded_impl!(isize, std::isize::MIN, std::isize::MAX);
                bounded_impl!(f32, std::f32::MIN, std::f32::MAX);
                bounded_impl!(f64, std::f64::MIN, std::f64::MAX);
                pub trait PrimInt: Num + Bounded + Eq + PartialOrd + Ord + Clone + Copy {}
                impl PrimInt for u32 {}
                impl PrimInt for usize {}
                impl PrimInt for i32 {}
                impl PrimInt for isize {}
            }
        }
    }
    pub mod debug {
        /// 変数をデバッグ出力する
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}
