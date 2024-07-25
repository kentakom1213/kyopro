#![allow(non_snake_case)]

use crate::cp_library_rs::{string::rolling_hash::RollingHash, utils::yesno::YesNo};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
        LR: [(Usize1, usize); Q]
    }

    let rhl = RollingHash::from_str(&S, BASE);
    let rhr = RollingHash::from_str(&S.chars().rev().collect::<String>(), BASE);

    for &(l, r) in &LR {
        let x = rhl.get(l, r);
        let y = rhr.get(N - r, N - l);
        println!("{}", (x == y).yesno());
    }
}

const BASE: usize = 20021213;

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod number_theory {
        pub mod modint_for_rollinghash {
            //! Modintの構造体
            #[rustfmt::skip]
            pub mod modint {
                pub const MOD: usize = (1 << 61) - 1;
                const MASK30: usize = (1 << 30) - 1;
                const MASK31: usize = (1 << 31) - 1;
                const MASK61: usize = MOD;
                fn mul(a: usize, b: usize) -> usize { let (au, ad) = (a >> 31, a & MASK31); let (bu, bd) = (b >> 31, b & MASK31); let m = ad * bu + au * bd; let (mu, md) = (m >> 30, m & MASK30); calcmod(au * bu * 2 + mu + (md << 31) + ad * bd) }
                fn calcmod(x: usize) -> usize { let xu = x >> 61; let xd = x & MASK61; let res = xu + xd; if res >= MOD { res - MOD } else { res } }
                use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
                #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint(pub usize);
                impl Modint { pub fn new(n: usize) -> Self { Self(calcmod(n)) } }
                impl Neg for Modint { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
                impl Add for Modint { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
                impl Sub for Modint { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
                impl Mul for Modint { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(mul(self.0, rhs.0)) } }
                impl Div for Modint { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
                impl AddAssign for Modint { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
                impl SubAssign for Modint { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
                impl MulAssign for Modint { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
                impl From<usize> for Modint { fn from(value: usize) -> Self { Modint::new(value) } }
                impl Add<usize> for Modint { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
                impl Sub<usize> for Modint { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
                impl Mul<usize> for Modint { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
                impl Div<usize> for Modint { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
                impl AddAssign<usize> for Modint { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
                impl SubAssign<usize> for Modint { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
                impl MulAssign<usize> for Modint { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
                impl Display for Modint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
                impl PartialEq<usize> for Modint { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
                impl FromStr for Modint { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { usize::from_str(s).map(Modint::new) } }
                pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
                impl Fp for Modint { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = mul(res, a); } a = mul(a, a); b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
                impl Sum<Modint> for Modint { fn sum<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(0), |acc, x| acc + x) } }
                impl Product<Modint> for Modint { fn product<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(1), |acc, x| acc * x) } }
            }
            pub use modint::*;
        }
    }
    pub mod string {
        pub mod rolling_hash {
            //! ローリングハッシュ
            use crate::cp_library_rs::number_theory::modint_for_rollinghash::modint::Modint;
            /// # RollingHash
            /// 文字列の比較を高速に行う
            /// - 計算量: `O(n+m)`
            #[derive(Debug)]
            pub struct RollingHash {
                pub size: usize,
                power: Vec<Modint>,
                hash: Vec<Modint>,
                base: Modint,
            }
            impl RollingHash {
                /// 初期化
                pub fn build(arr: &[Modint], base: usize) -> Self {
                    let size = arr.len();
                    let mut power = vec![Modint(0); size + 1];
                    let mut hash = vec![Modint(0); size + 1];
                    // hashを初期化
                    let (mut h, mut p) = (Modint(0), Modint(1));
                    for i in 0..size {
                        h = arr[i] + (h * base);
                        p *= base;
                        hash[i + 1] = h;
                        power[i + 1] = p;
                    }
                    Self {
                        size,
                        power,
                        hash,
                        base: base.into(),
                    }
                }
                /// 文字列から生成
                pub fn from_str(s: &str, base: usize) -> Self {
                    let arr: Vec<Modint> = s.chars().map(Self::ord).map(Modint).collect();
                    Self::build(&arr, base)
                }
                /// `l..r`のハッシュを取得
                /// - 計算量: `O(1)`
                pub fn get(&self, l: usize, r: usize) -> Modint {
                    self.hash[r] - self.hash[l] * self.power[r - l]
                }
                /// `0..size`のハッシュを取得
                /// - 計算量: `O(1)`
                pub fn full(&self) -> Modint {
                    self.hash[self.size]
                }
                /// a,bからの最長共通接頭辞の長さを調べる
                /// - 計算量: `O(log N)`
                pub fn getLCP(&self, a: usize, b: usize) -> usize {
                    let len = self.size.saturating_sub(a.max(b));
                    let (mut lo, mut hi) = (0, len + 1);
                    while hi - lo > 1 {
                        let mid = (lo + hi) / 2;
                        if self.get(a, a + mid) == self.get(b, b + mid) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    lo
                }
                /// ハッシュ同士を連結
                /// - 計算量: `O(1)`
                pub fn concat(&self, h1: Modint, h2: Modint, h2_len: usize) -> Modint {
                    h1 * self.power[h2_len] + h2
                }
                /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
                #[inline]
                fn ord(c: char) -> usize {
                    let a = 'A' as u32;
                    let c = c as u32;
                    (c - a) as usize
                }
            }
        }
    }
    pub mod utils {
        pub mod yesno {
            //! boolから"Yes"/"No"への変換
            pub trait YesNo {
                /// `true`->`"Yes"`, `false`->`"No"` に変換
                fn yesno(&self) -> String;
            }
            impl YesNo for bool {
                fn yesno(&self) -> String {
                    if *self {
                        "Yes".to_string()
                    } else {
                        "No".to_string()
                    }
                }
            }
        }
    }
}
