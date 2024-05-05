//                  B - 円錐                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc052/tasks/arc052_b
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
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
    };
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
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const PI: f64 = 3.141592653589;

/// # 円錐
struct Cone {
    bottom: f64,
    top: f64,
    height: f64,
    vol: f64,
}

impl Cone {
    fn new(x: f64, r: f64, h: f64) -> Self {
        Cone {
            bottom: x,
            top: x + h,
            height: h,
            vol: PI * r * r * h / 3.0,
        }
    }

    /// x座標が[a,b]の部分の堆積を求める
    fn partial_vol(&self, a: f64, b: f64) -> f64 {
        // 重複部分があるか判定
        if a.max(self.bottom) <= b.min(self.top) {
            if a <= self.bottom && self.top <= b {
                self.vol
            } else if self.bottom <= a && self.top <= b {
                self.vol * ((self.top - a) / self.height).powf(3.0)
            } else {
                let bottom = a.max(self.bottom);
                let big = self.vol * ((self.top - bottom) / self.height).powf(3.0);
                let small = self.vol * ((self.top - b) / self.height).powf(3.0);
                big - small
            }
        } else {
            0.0
        }
    }
}

/// ## 方針
/// - N <= 100
/// - Q <= 10^5
/// であるから、O(NQ)が間に合う。
/// →愚直実装でOK！
fn main() {
    let (N, Q) = get!(usize, usize);
    let cones: Vec<Cone> = get!(f64, f64, f64; N)
        .iter()
        .map(|&(x, r, h)| Cone::new(x, r, h))
        .collect();
    let queries = get!(f64, f64; Q);

    for &(a, b) in &queries {
        let mut res = 0.0;
        for cone in &cones {
            res += cone.partial_vol(a, b);
        }
        println!("{}", res);
    }
}
