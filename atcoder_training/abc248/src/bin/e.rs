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
        K: usize,
        XY: [(isize, isize); N]
    }

    if K == 1 {
        println!("Infinity");
        return;
    }

    let mut used = vec![vec![false; N]; N];
    let mut ans = 0;

    // 直線を全列挙
    for i in 0..N {
        for j in i + 1..N {
            if used[i][j] {
                continue;
            }
            let (a, b) = (XY[i], XY[j]);
            // 上に乗っている点をカウント
            let colinear = (0..N).filter(|&x| is_on_line((a, b), XY[x])).collect_vec();
            let cnt = colinear.len();

            if cnt >= K {
                ans += 1;
            }
            // 記録
            for ii in 0..cnt {
                for jj in ii + 1..cnt {
                    used[colinear[ii]][colinear[jj]] = true;
                }
            }
        }
    }

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;

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

/// 点xが直線ab上に存在するか
pub fn is_on_line(ab: Line<isize>, x: Pos<isize>) -> bool {
    let (a, b) = ab;
    let xa = a.sub(x);
    let xb = b.sub(x);
    xa.0 * xb.1 == xa.1 * xb.0
}
