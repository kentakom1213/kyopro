//            F - Push and Carry
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc323/tasks/abc323_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
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

use std::{cmp::Ordering::*, mem::swap};
use std::ops::{Add, Mul, Neg, Sub};

type Pos<T> = (T, T);
type Line<T> = (Pos<T>, Pos<T>);

trait Vec2<T> {
    fn mul(&self, scalar: T) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    fn dot(&self, other: Self) -> T;
    fn cross(&self, other: Self) -> T;
    fn l2(&self, other: Self) -> T;
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
    /// ## cross
    /// ベクトルのクロス積
    fn cross(&self, other: Self) -> T {
        (self.0 * other.1) - (other.0 * self.1)
    }
    /// ## l2
    /// ユークリッド距離の2乗の値を返す
    fn l2(&self, other: Self) -> T {
        (self.0 - other.0) * (self.0 - other.0) + (self.1 - other.1) * (self.1 - other.1)
    }
}

/// ## is_collided
/// 線分abと線分xyが衝突しているかどうか
fn is_collided(ab: Line<isize>, xy: Line<isize>) -> bool {
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

/// ## l1
/// l1ノルム（マンハッタン距離）を求める
fn l1(a: Pos<isize>, b: Pos<isize>) -> isize {
    let d = a.sub(b);
    d.0.abs() + d.1.abs()
}

fn main() {
    input! {
        S: (isize, isize),
        T: (isize, isize),
        U: (isize, isize)
    }

    // 座標を平行移動、反転し、
    // - Zが原点
    // - Yが第1象限
    // になるように調整

    // 平行移動
    let mut S = S.sub(U);
    let mut T = T.sub(U);

    // 反転
    if T.0 < 0 {
        S.0 *= -1;
        T.0 *= -1;
    }
    if T.1 < 0 {
        S.1 *= -1;
        T.1 *= -1;
    }

    debug!(S, T);

    let mut ans = 1001001001001001001;

    for _ in 0..2 {
        // 荷物を下に押す
        let (step1, (x1, x2), (y1, y2)) = push_down(S, T);
        // 軸を入れ替え、下に押す
        let (step2, ..) = push_down((x2, x1), (y2, y1));
        chmin!(
            ans,
            step1 + step2
        );
        // 軸を入れ替えてもう一回
        swap(&mut S.0, &mut S.1);
        swap(&mut T.0, &mut T.1);
    }

    println!("{ans}");
}

/// 高橋くんがX、荷物がYにあるとき、
/// - 荷物を(T.0, 0)に移動するまでの最小行動回数
/// - 移動後の高橋くんの座標
/// - 移動後の荷物の座標
///
/// を返す
fn push_down(S: Pos<isize>, T: Pos<isize>) -> (isize, Pos<isize>, Pos<isize>) {
    // T.1 == 0なら何もしない
    if T.1 == 0 {
        return (0, S, T);
    }
    // 高橋くんを(T.0, T.1 + 1)に移動
    let mut step = (S.0 - T.0).abs() + (S.1 - (T.1 + 1)).abs();
    // 荷物を迂回する必要がある場合+2
    if S.0 == T.0 && S.1 < T.1 {
        step += 2;
    }
    // 下に押す
    step += T.1;

    (step, (T.0, 1), (T.0, 0))
}
