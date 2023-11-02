// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use std::ops::{Add, Sub, Mul, Neg};

type Pos<T> = (T, T);
type Line<T> = (Pos<T>, Pos<T>);
type P = Pos<isize>;

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

/// ## 方針
/// - 傾きによって分類し、組の数が最も小さくなるようにする
fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N],
    }

    if N == 1 {
        println!("1");
        return;
    }

    let mut ans = INF;

    for i in 0..N {
        for j in i+1..N {
            let t = XY[i].sub(XY[j]);
            let cnt = divide(&XY, t);
            if cnt < ans {
                ans = cnt;
            }
        }
    }

    println!("{}", ans);
}

/// ## is_on_line
fn is_on_line(a: P, b: P, tilt: P) -> bool {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (tx, ty) = tilt;

    tx * (ay - by) == ty * (ax - bx)
}

/// ## divide
/// 傾きと点の集合を与えると、それらをグループに分類する
fn divide(XY: &Vec<P>, tilt: P) -> usize {
    let mut groups = BTreeSet::new();

    'out: for &b in XY {
        if groups.is_empty() {
            groups.insert(b);
        }
        else {
            for &a in &groups {
                if is_on_line(a, b, tilt) {
                    continue 'out;
                }
            }
            groups.insert(b);
        }
    }

    groups.len()
}
