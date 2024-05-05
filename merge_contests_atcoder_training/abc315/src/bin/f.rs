// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use alga::general::Lattice;
// imports
use itertools::Itertools;
use num_traits::Pow;
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

fn main() {
    input! {
        N: usize,
        P: [(f64, f64); N]
    }

    // dp[i][j] := i番目のチェックポイントに(i-j)番目のチェックポイントから到達したときのコストの最小値
    let mut dp = vec![vec![INF; 60]; N];
    dp[0][0] = 0.0;

    for i in 1..N {
        for j in (0..i).rev() {
            let skip = i - j - 1;
            for k in 0..60_usize.saturating_sub(skip) {
                chmin! {
                    dp[i][k + skip],
                    dp[j][k] + P[i].dist2(P[j]).sqrt()
                };
            }
        }
    }

    if cfg!(debug_assertions) {
        for row in &dp {
            eprintln!("{:?}", row);
        }
    }

    let mut ans = INF;
    for i in 0..60 {
        let mut pen = 0.0;
        if i > 0 {
            pen = 2.0.pow((i - 1) as f64);
        }
        chmin! {
            ans,
            dp[N - 1][i] + pen
        };
    }
    println!("{:.20}", ans);
}

const INF: f64 = 1e20;

use std::ops::{Add, Mul, Neg, Sub};
pub type Pos<T> = (T, T);
pub type Line<T> = (Pos<T>, Pos<T>);
/// ベクトル演算を行う
pub trait Vec2<T> {
    fn mul(&self, scalar: T) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    /// ドット積
    fn dot(&self, other: Self) -> T;
    /// クロス積
    fn cross(&self, other: Self) -> T;
    /// L2-ノルム（の2乗）
    fn dist2(&self, other: Self) -> T;
}
impl<T> Vec2<T> for Pos<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg,
{
    fn mul(&self, scalar: T) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }
    fn add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
    fn sub(&self, other: Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
    fn dot(&self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }
    fn cross(&self, other: Self) -> T {
        (self.0 * other.1) - (other.0 * self.1)
    }
    fn dist2(&self, other: Self) -> T {
        (self.0 - other.0) * (self.0 - other.0) + (self.1 - other.1) * (self.1 - other.1)
    }
}
/// 線分abと線分xyが衝突しているかどうか
pub fn is_collided(ab: Line<isize>, xy: Line<isize>) -> bool {
    let (a, b) = ab;
    let (x, y) = xy;
    // Aから見たとき
    let AX = x.sub(a);
    let AY = y.sub(a);
    let AB = b.sub(a);
    // Xから見たとき
    let XA = a.sub(x);
    let XB = b.sub(x);
    let XY = y.sub(x);
    AB.cross(AX) * AB.cross(AY) < 0 && XY.cross(XA) * XY.cross(XB) < 0
}
