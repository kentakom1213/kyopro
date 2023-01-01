//                B - 格子点と整数               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc018/tasks/arc018_2
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

/// Vector2D
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    fn mul(&self, scalar: isize) -> Self {
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

    fn dot(&self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }
}

// solve
fn main() {
    let N = get!(usize);
    let ps: Vec<Vec2> = get!(isize, isize; N)
        .iter()
        .map(|&(x, y)| Vec2::new(x, y))
        .collect();

    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                let (a, b, c) = (ps[i], ps[j], ps[k]);

                if is_ok(a, b, c) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

/// 3点によって作られる三角形の面積が整数になるかを判定する
fn is_ok(a: Vec2, b: Vec2, c: Vec2) -> bool {
    let ab = b.sub(a);
    let ac = c.sub(a);
    let S2 = (ab.x * ac.y - ac.x * ab.y).abs();  // 面積の2倍

    S2 != 0 && S2 % 2 == 0
}
