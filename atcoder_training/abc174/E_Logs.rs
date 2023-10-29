//                 E - Logs                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc174/tasks/abc174_e
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

/// ## 方針
/// - 丸太の断片の長さ最大値を`x`にすることができるかという判定問題
/// - 二分探索で最小値を求める
fn main() {
    let (N, K) = get!(usize, usize);
    let A = get!(usize;;);

    let (mut lo, mut hi) = (0, INF);
    while (hi - lo) > 1 {
        let m = (lo + hi) / 2;
        if can(&A, K, m) {
            hi = m;
        } else {
            lo = m;
        }
    }

    println!("{}", hi);
}

/// `K`回以内切ることで、丸太の断片の長さ最大値を`x`にすることができるか
/// - 長さ`a`の丸太を最大`x`になるように切るときの最小回数は
/// - 
fn can(logs: &[usize], K: usize, x: usize) -> bool {
    let mut cnt = 0;
    for &a in logs {
        cnt += (a - 1) / x;
    }

    cnt <= K
}
