//                 D - 一刀両断                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc016/tasks/abc016_4
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
macro_rules! get {
    ($t:ty) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
    }};
    ($($t:ty),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
    }};
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
    }};
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

use std::ops::{Add, Sub, Mul, Neg};

/// Vector2D
/// - 二次元ベクトル
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T>
where T: Copy
       + Add<Output = T>
       + Sub<Output = T>
       + Mul<Output = T>
       + Neg
{
    fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    fn mul(&self, scalar: T) -> Self {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn add(&self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// ## cross
    /// ベクトルのクロス積
    fn cross(&self, other: Self) -> T {
        (self.x * other.y) - (other.x * self.y)
    }

    /// ## dist2
    /// 距離の2乗の値を返す
    fn dist2(&self, other: Self) -> T {
        (self.x - other.x) * (self.x - other.x)
        + (self.y - other.y) * (self.y - other.y)
    }
}

type Line = (
    Vec2<isize>,
    Vec2<isize>
);

/// ## is_collided
/// 線分abと線分xyが衝突しているかどうか
fn is_collided(ab: Line, xy: Line) -> bool {
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// 多角形の辺との交点の数を`n`個とすると、
/// 求める値は`n/2 + 1`
fn main() {
    let (ax, ay, bx, by) = get!(isize, isize, isize, isize);
    let N = get!(usize);
    let mut xy = get!(isize, isize; N);
    xy.push(xy[0]);
    
    // 線分
    let ab = (
        Vec2::new(ax, ay),
        Vec2::new(bx, by)
    );

    let mut cnt = 0;

    for i in 0..N {
        let x = Vec2::new(xy[i].0, xy[i].1);
        let y = Vec2::new(xy[i+1].0, xy[i+1].1);

        if is_collided(ab, (x, y)) {
            cnt += 1;
        }
    }

    let ans = cnt / 2 + 1;
    println!("{}", ans);
}
