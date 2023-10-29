//        D - Anything Goes to Zero        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/aising2020/tasks/aising2020_d
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

// solve
fn main() {
    let N = get!(usize);
    let mut pc: usize = 0;
    let X: Vec<usize> = get!(String)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .inspect(|v| pc += *v)
        .rev()
        .collect();

    // fの前計算（dp）
    let mut cnt: Vec<usize> = vec![0; 201010];
    for i in 1_usize..201010 {
        let pp = i.count_ones() as usize;
        cnt[i] = cnt[i % pp] + 1;
    }

    let (pcp, pcn) = (pc.saturating_sub(1), pc + 1);

    let (mut tot1, mut tot2) = (0, 0);
    let (mut p1, mut p2) = (1, 1);
    for i in 0..N {
        if X[i] == 1 {
            tot1 = (tot1 + p1) % pcn;
            if 2 <= pc { tot2 = (tot2 + p2) % pcp; }
        }
        p1 = (p1 * 2) % pcn;
        if 2 <= pc { p2 = (p2 * 2) % pcp; }
    }

    // 解を計算
    let mut ans = vec![];
    let (mut p1, mut p2) = (1, 1);
    for i in 0..N {
        if X[i] == 1 {
            if pc == 1 {
                ans.push(0);
            } else {
                let x = (pcp + tot2 - p2) % pcp;
                ans.push(cnt[x] + 1);
            }
        } else {
            let x = (pcn + tot1 + p1) % pcn;
            ans.push(cnt[x] + 1);
        }
        p1 = (p1 * 2) % pcn;
        if 2 <= pc { p2 = (p2 * 2) % pcp; }
    }

    for &v in ans.iter().rev() {
        println!("{}", v);
    }
}
