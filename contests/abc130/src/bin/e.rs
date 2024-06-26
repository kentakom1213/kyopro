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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

use crate::{bit_2d::BIT2D, monoid::examples::Add, monoid_mod::ModAdd};

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [usize; N],
        T: [usize; M]
    }

    let mut bit = BIT2D::<ModAdd<1_000_000_007>>::new(N + 1, M + 1);

    bit.add(0, 0, 1.into());

    for i in 0..N {
        for j in 0..M {
            if S[i] != T[j] {
                continue;
            }
            let tmp = bit.prefix_sum(i + 1, j + 1);
            bit.add(i + 1, j + 1, tmp);
        }
    }

    let ans = bit.prefix_sum(N + 1, M + 1);

    println!("{ans}");
}

mod modint {
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
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)] pub struct Modint<const MOD: usize>(pub usize);
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
        impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
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
        impl<const MOD: usize> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
        pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
        impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
        impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
        impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
    }
}

mod monoid {
    //! モノイド
    use std::fmt::Debug;
    /// モノイド
    pub trait Monoid {
        /// 元の型
        type Val: Debug + Clone + PartialEq;
        /// 単位元
        const E: Self::Val;
        /// 演算
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
    }
    /// 各種モノイド
    pub mod examples {
        use super::Monoid;
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            const E: Self::Val = 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = isize;
            const E: Self::Val = (1 << 31) - 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = isize;
            const E: Self::Val = -((1 << 31) - 1);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            const E: Self::Val = 0;
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
        struct Affine;
        impl Monoid for Affine {
            type Val = (f64, f64);
            const E: Self::Val = (1.0, 0.0);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                let &(a1, b1) = left;
                let &(a2, b2) = right;
                (a2 * a1, a2 * b1 + b2)
            }
        }
    }
}

mod monoid_mod {
    //! modを取るモノイド
    use crate::modint::*;
    use crate::monoid::Monoid;
    /// あまりをとる和
    pub struct ModAdd<const MOD: usize>;
    impl<const MOD: usize> Monoid for ModAdd<MOD> {
        type Val = Modint<MOD>;
        const E: Self::Val = Modint::<MOD>(0);
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left + *right
        }
    }
    /// あまりをとる積
    pub struct ModMul<const MOD: usize>;
    impl<const MOD: usize> Monoid for ModMul<MOD> {
        type Val = Modint<MOD>;
        const E: Self::Val = Modint::<MOD>(1);
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left * *right
        }
    }
    /// アフィン変換
    pub struct Affine<const MOD: usize>;
    impl<const MOD: usize> Monoid for Affine<MOD> {
        type Val = (Modint<MOD>, Modint<MOD>);
        const E: Self::Val = (Modint::<MOD>(1), Modint::<MOD>(0));
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            let &(a1, b1) = left;
            let &(a2, b2) = right;
            (a2 * a1, a2 * b1 + b2)
        }
    }
}

mod bit_2d {
    //! 2次元BIT
    /// cfor! {}
    macro_rules! cfor {
        ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
            $def
            while $fin {
                $bl
                $incr
            }
        }}
    }
    use crate::monoid::Monoid;
    pub struct BIT2D<M: Monoid> {
        pub H: usize,
        pub W: usize,
        pub data: Vec<Vec<M::Val>>,
    }
    impl<M: Monoid> BIT2D<M> {
        #[inline]
        fn lsb(x: usize) -> usize {
            x & x.wrapping_neg()
        }
        /// 2次元BITを作成する
        pub fn new(H: usize, W: usize) -> Self {
            Self {
                H,
                W,
                data: vec![vec![M::E; W + 1]; H + 1],
            }
        }
        /// 位置 (r,c) に値 `v` を加算する
        /// - `(r, c)`: 加算を行うインデックス（`0-indexed`）
        /// - `x`: 加算する値
        pub fn add(&mut self, mut r: usize, mut c: usize, v: M::Val) {
            // 0-indexedに修正
            r += 1;
            c += 1;
            cfor! {let mut i = r; i <= self.H; i += Self::lsb(i) ;; {
                cfor! {let mut j = c; j <= self.W; j += Self::lsb(j) ;; {
                    self.data[i][j] = M::op(&self.data[i][j], &v);
                }}
            }}
        }
        /// 左上からの和を求める
        /// - `(r,c)`: 領域 `0<=i<r, 0<=j<c` に対しての総和（`0-indexed`）
        pub fn prefix_sum(&self, r: usize, c: usize) -> M::Val {
            let mut res = M::E;
            cfor! {let mut i = r; i > 0; i -= Self::lsb(i) ;; {
                cfor! {let mut j = c; j > 0; j -= Self::lsb(j) ;; {
                    res = M::op(&res, &self.data[i][j]);
                }}
            }}
            res
        }
    }
}
