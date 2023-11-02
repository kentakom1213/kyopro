// https://atcoder.jp/contests/abc266/tasks/abc266_c

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

use std::ops::{Add, Sub, Mul, Neg};

type T = isize;
type Pos<T> = (T, T);
type Line<T> = (Pos<T>, Pos<T>);

trait Vec2<T> {
    fn mul(&self, scalar: T) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    fn dot(&self, other: Self) -> T;
    fn cross(&self, other: Self) -> T;
    fn dist2(&self, other: Self) -> T;
}

impl<T> Vec2<T> for Pos<T>
where T: Copy
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Neg
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
    /// ## dist2
    /// 距離の2乗の値を返す
    fn dist2(&self, other: Self) -> T {
        (self.0 - other.0) * (self.0 - other.0)
        + (self.1 - other.1) * (self.1 - other.1)
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

// solve
fn main() {
    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        d: (isize, isize),
    }

    let is_ok = is_collided((a, b), (c, d)) ^ is_collided((a, c), (b, d)) == true;

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}