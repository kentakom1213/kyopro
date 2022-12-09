//           F - Distance Sums 2           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc220/tasks/abc220_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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

/// # 方針
/// LCAを使って最短経路を求める
fn main() {
    let N = get!(usize);
    let G = {
        let mut G = vec![vec![]; N];
        for _ in 0..N-1 {
            let (u, v) = get!(usize, usize);
            G[u-1].push(v-1);
            G[v-1].push(u-1);
        }
        G
    };

    // 頂点0からの距離を求める（bfs）
    let dist = {
        let mut dist = vec![INF; N];
        dist[0] = 0;
        let mut q = VecDeque::new();
        q.push_back(0);
        while let Some(u) = q.pop_front() {
            for &v in &G[u] {
                if dist[v] == INF {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
        dist
    };

    
}

trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as isize;
        while (ok - ng) > 1 {
            let mid = ((ng + ok) as usize) / 2;
            if v <= self[mid] {
                ok = mid as isize;
            } else {
                ng = mid as isize;
            }
        }
        ok as usize
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as isize;
        while (ok - ng) > 1 {
            let mid = ((ng + ok) as usize) / 2;
            if v < self[mid] {
                ok = mid as isize;
            } else {
                ng = mid as isize;
            }
        }
        ok as usize
    }
}
