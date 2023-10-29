//            C - Many Segments            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc207/tasks/abc207_c
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

type Range = (f64, f64);

// solve
fn main() {
    let N = get!(usize);
    let ranges: Vec<Range> = get!(usize, usize, usize; N)
        .iter()
        .map(|(t, l, r)| (*t, *l as f64, *r as f64))
        .map(|(t, l, r)|
            if t == 1 {
                (l, r)
            } else if t == 2 {
                (l, r - 0.5)
            } else if t == 3 {
                (l + 0.5, r)
            } else {
                (l + 0.5, r - 0.5)
            }
        )
        .collect();

    // 全ての組を探索
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            let (l1, r1) = ranges[i];
            let (l2, r2) = ranges[j];
            if l1.max(l2) <= r1.min(r2) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
