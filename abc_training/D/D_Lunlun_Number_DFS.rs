//            D - Lunlun Number            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc161/tasks/abc161_d
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
const BASE: usize = 9;

// solve
fn main() {
    let K = get!(usize);
    
    let mut all: Vec<usize> = vec![];
    for i in 1..=9 {
        dfs(10, i, &mut all);
    }

    all.sort();

    let ans = all[K-1];
    println!("{}", ans);
}

/// ## DFS
/// Lunlun数を列挙
fn dfs(lim: usize, val: usize, all: &mut Vec<usize>) {
    all.push(val);

    // limがなくなったら打ち切り
    if lim == 0 { return; }

    // 次の桁に進む
    let tail = val % 10;  // 末尾の数
    for i in tail.saturating_sub(1)..=(tail+1).min(9) {
        let nxt = val * 10 + i;
        // 再帰呼び出し
        dfs(lim-1, nxt, all);
    }
}
