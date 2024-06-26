#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::vec2::Vec2;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        P: [(isize, isize); N]
    }

    for i in 0..N {
        let mut maxi = 0;
        let mut res = 0;
        for j in 0..N {
            if i == j {
                continue;
            }
            let d = P[i].dist2(P[j]);
            if d > maxi {
                maxi = d;
                res = j;
            }
        }
        println!("{}", res + 1);
    }
}

mod vec2 {
    //! 幾何ライブラリ
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
}
