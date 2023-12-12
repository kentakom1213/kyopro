// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    // 全探索
    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let a = xy[j].sub(xy[i]);
                let b = xy[k].sub(xy[i]);
                let double = a.0 * b.1 - a.1 * b.0;
                if double != 0 && double % 2 == 0 {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}

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
