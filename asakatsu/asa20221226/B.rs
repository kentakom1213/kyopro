// https://atcoder.jp/contests/abc207/tasks/abc207_c

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

type Range = (usize, usize, usize);

// solve
fn main() {
    let N = get!(usize);
    let ranges: Vec<Range> = get!(usize, usize, usize; N)
        .iter()
        .map(|&(t, l, r)| (t-1, l, r))
        .collect();

    let mut ans = 0;

    // ペアを全探索
    for i in 0..N {
        for j in i+1..N {
            if has_common_part(ranges[i], ranges[j]) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

/// 区間が共通部分を持つかどうか
fn has_common_part(l: Range, r: Range) -> bool {
    let L = (
        (l.1 as f64 + if (l.0 >> 1) & 1 == 0 {0.0} else {0.5}),
        (l.2 as f64 - if l.0 & 1 == 0 {0.0} else {0.5})
    );
    let R = (
        (r.1 as f64 + if (r.0 >> 1) & 1 == 0 {0.0} else {0.5}),
        (r.2 as f64 - if r.0 & 1 == 0 {0.0} else {0.5})
    );

    // 共通部分の判定
    (L.0 < R.1) ^ (L.1 < R.0)
}
