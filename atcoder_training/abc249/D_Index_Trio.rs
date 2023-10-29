//              D - Index Trio             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc249/tasks/abc249_d
// ----------------------------------------

/* # 方針
 * Ai == Aj*Ak
 * ソートしてからj, kのペアを探索することで計算量を調和級数の和に帰着できる
 */

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1_000_000_000_000_000_000;
static MAX: usize = 202020;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    let mut cnt = vec![0; MAX];
    for &a in &A {
        cnt[a] += 1;
    }

    let mut ans: usize = 0;

    for i in 1..MAX {
        for j in 1..MAX {
            if i*j >= MAX { break; }
            ans += cnt[i] * cnt[j] * cnt[i*j];
        }
    }

    println!("{}", ans);
}

