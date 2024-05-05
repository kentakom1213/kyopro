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

use num_traits::Pow;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use vec2::{Pos, Vec2};

const EPS: f64 = 1e-8;

fn main() {
    input! {
        N: usize,
        XY: [(f64, f64); N]
    }

    let isok = |r: f64| -> bool {
        // 2点を列挙
        for i in 0..N {
            for j in i + 1..N {
                let a = XY[i];
                let b = XY[j];
                // 円同士に交点が存在しない場合
                if a.dist2(b).sqrt() > 2.0 * r {
                    continue;
                }
                // 距離rの点
                let [X, Y] = circle_interesction(a, b, r);
                debug!(X, Y);

                for o in [X, Y] {
                    let ok = (0..N)
                        .filter(|&k| k != i && k != j)
                        .all(|k| o.dist2(XY[k]).sqrt() <= r + EPS);

                    if ok {
                        return true;
                    }
                }
            }
        }
        false
    };

    let mut ng = 0.0;
    let mut ok = 1e9;

    for _ in 0..50 {
        let mid = (ng + ok) * 0.5;
        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok:.10}");
}

/// 点A,Bを中心とする半径rの円の交点X,Yを求める
fn circle_interesction(A: Pos<f64>, B: Pos<f64>, r: f64) -> [Pos<f64>; 2] {
    // 中点H
    let H = A.add(B).mul(0.5);
    // 線分ABの距離
    let Dab = A.dist2(B).sqrt();
    // 線分XHの距離
    let Dxh = (r.powf(2.0) - (Dab * 0.5).powf(2.0)).sqrt();
    // 線分ABの法線ベクトル
    let (dx, dy) = B.sub(A);
    let v = (-dy, dx).mul(1.0 / Dab);

    let X = H.add(v.mul(Dxh));
    let Y = H.add(v.mul(-Dxh));

    [X, Y]
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
